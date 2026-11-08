//! Problem picker — fetches questions, writes local files, and drives submission.
//!
//! [`Picker`] is the main orchestrator used by both the CLI commands (`pick`,
//! `submit`, `test`) and the TUI. It wraps [`LeetCodeClient`] and adds local
//! file I/O and a disk cache for the problem list.
use crate::cache::CacheService;
use crate::config::{CONFIG, solutions_dir};
use crate::error::EngineError;
use crate::format::format_result;
use crate::models::{Identifier, ProblemSummary, UserDetail};
use crate::services::submission::SubmissionService;
use crate::{
    client::{LeetCodeApi, LeetCodeClient},
    models::Language,
};
use std::fs;

/// Orchestrates problem fetching, file generation, submission, and caching.
///
/// Constructed once per command invocation and shared across async tasks via
/// clone (the inner [`LeetCodeClient`] is `Clone`).
#[derive(Clone)]
pub struct Picker<C = LeetCodeClient> {
    pub client: C,
    pub cache: CacheService,
}

impl<C> Picker<C> {
    pub fn new(client: C) -> Self {
        Picker {
            client,
            cache: CacheService::new(),
        }
    }

    pub fn with_cache(client: C, cache: CacheService) -> Self {
        Picker { client, cache }
    }
}

impl<C: LeetCodeApi + Clone + Send + Sync + 'static> Picker<C> {
    /// Resolves a problem by [`Identifier`], writes the Markdown description
    /// and language-specific code stub to disk, and returns their paths.
    ///
    /// # Existing Files & Migration Policy
    /// If both code and description files already exist on disk (slug-based match),
    /// they are returned immediately without network access. Existing files are
    /// intentionally left unmodified to safeguard user code. Legacy files containing
    /// the older internal `question_id` in their comment headers remain fully
    /// compatible because submission and testing resolve the problem metadata
    /// dynamically via the filename slug, not from the comment header.
    ///
    /// Newly generated files will have `question_frontend_id` in their headers.
    ///
    /// # Returns
    /// `(code_file_path, description_file_path)` on success.
    pub async fn pick(
        &self,
        identifier: &Identifier,
        language: &Option<Language>,
    ) -> crate::error::Result<(String, String)> {
        let mut language = match language {
            Some(lang) => lang.clone(),
            None => {
                let config = CONFIG.get().expect("Config not initialised");
                if let Some(lang) = &config.language {
                    Language::from(lang)
                } else {
                    println!("🔤 No language specified, defaulting to Python.");
                    Language::Python
                }
            }
        };

        // Early-return if files already exist: we preserve existing user code as-is
        // without rewriting comment headers.
        let dir = solutions_dir();

        let cached_slug = if let Identifier::Number(num) = identifier {
            if let Ok(Some(problems)) = self.cache.read::<Vec<ProblemSummary>>("data.json") {
                problems.into_iter().find(|p| p.id == *num).map(|p| p.slug)
            } else {
                None
            }
        } else {
            None
        };

        let early_slug = match identifier {
            Identifier::String(ident) => Some(ident.as_str()),
            Identifier::Number(_) => cached_slug.as_deref(),
        };

        if let Some(slug) = early_slug {
            let snake_slug = slug.replace("-", "_");
            let code_path = dir.join(format!("{}.{}", snake_slug, language.code_extension()));
            let desc_path = dir.join(format!("{}.md", snake_slug));
            if code_path.exists() && desc_path.exists() {
                return Ok((
                    code_path.display().to_string(),
                    desc_path.display().to_string(),
                ));
            }
        }

        if !dir.as_os_str().is_empty() {
            fs::create_dir_all(&dir).map_err(|e| {
                eprintln!(
                    "❌ failed to create solutions directory {}: {}",
                    dir.display(),
                    e
                );
                EngineError::System
            })?;
        }

        let question = match identifier {
            Identifier::Number(num) => {
                println!("🔍 Fetching problem ID: {}...", num);
                self.client.get_question_by_id(*num).await?
            }
            Identifier::String(identifier) => {
                println!("🔍 Fetching problem: {}...", identifier);
                self.client.get_question_by_slug(identifier).await?
            }
        };

        // Convert LeetCode's raw HTML into wrapped terminal text (80 columns wide)
        let formatted_content = html2md::parse_html(&question.content);
        let md_content = format!("# {}\n\n{}", question.title, formatted_content);

        let snippet = question
            .code_snippets
            .iter()
            .find(|s| s.lang_slug == language.to_lang_slug());

        let snippet = match snippet {
            Some(s) => s,
            None => {
                let snippet = question.code_snippets.first().ok_or_else(|| {
                    EngineError::Other("LeetCode problem has no code snippets".to_string())
                })?;
                language = Language::from(snippet.lang_slug.clone());
                snippet
            }
        };

        // determine filenames (converting kebab-case to snake_case)
        let snake_slug = question.title_slug.replace("-", "_");
        let code_path = dir.join(format!("{}.{}", snake_slug, language.code_extension()));
        let desc_path = dir.join(format!("{}.md", snake_slug));

        // Never overwrite an existing code file: protect user code.
        let mut generated = false;
        if code_path.exists() {
            println!("📄 Keeping existing {}", code_path.display());
        } else {
            // Write metadata comment at the top of the generated code file.
            // We use `question_frontend_id` (the public problem number on LeetCode) so the file
            // header displays the familiar number matching the website and CLI search.
            let meta = format!(
                "{} id={} slug={} lang={}",
                language.meta_comment_prefix(),
                question.question_frontend_id,
                question.title_slug,
                language.to_lang_slug()
            );

            if let Err(e) = fs::write(&code_path, format!("{}\n\n{}", meta, snippet.code)) {
                eprintln!("❌ failed to write code file: {}", e);
                return Err(EngineError::System);
            }
            generated = true;
        }

        if desc_path.exists() {
            println!("📄 Keeping existing {}", desc_path.display());
        } else {
            if let Err(e) = fs::write(&desc_path, md_content) {
                eprintln!("❌ failed to write description file: {}", e);
                return Err(EngineError::System);
            }
            generated = true;
        }

        if generated {
            println!("✅ files generated successfully.");
        }

        Ok((
            code_path.display().to_string(),
            desc_path.display().to_string(),
        ))
    }

    /// Runs the solution file against `custom_cases`, or the problem's built-in
    /// example test cases when `None`, and prints the result, but **does not**
    /// record it as an official submission.
    pub async fn test_submit(&self, file: &str, custom_cases: Option<&str>) {
        let service = SubmissionService::new(self.client.clone());
        match service.submit_or_test(file, true, custom_cases).await {
            Ok(result) => format_result(&result),
            Err(e) => eprintln!("❌ {}", e),
        }
    }

    /// Submits the solution file to LeetCode for full judging and prints the
    /// verdict, test-case counts, and performance percentiles.
    pub async fn submit(&self, file: &str) {
        let service = SubmissionService::new(self.client.clone());
        match service.submit_or_test(file, false, None).await {
            Ok(result) => format_result(&result),
            Err(e) => eprintln!("❌ {}", e),
        }
    }

    /// Returns the cached user profile, refreshing it in the background.
    ///
    /// **Cache-aside strategy:**
    /// 1. Read `user.json` from disk.
    /// 2. If found, return it immediately and spawn a background task that
    ///    fetches the latest data from the API and overwrites the file.
    /// 3. If not found, block on the API fetch, write the file, then return.
    pub async fn get_user_data(&self) -> crate::error::Result<UserDetail> {
        let user_path = self.cache.user_path();
        match fs::read_to_string(&user_path) {
            Ok(v) => {
                let client = self.client.clone();
                let user_path_bg = user_path.clone();
                tokio::spawn(async move {
                    let result: Result<(), Box<dyn std::error::Error + Send + Sync>> = async {
                        let user_detail = client.get_user_detail().await?;
                        let data = serde_json::to_string(&user_detail)?;
                        let _ = fs::write(&user_path_bg, &data);
                        Ok(())
                    }
                    .await;

                    let _ = result;
                });

                let user_detail: UserDetail = serde_json::from_str(&v).map_err(|e| {
                    eprintln!("Failed to parse user details: {}", e);
                    eprintln!("Try running `leetrs tui` again to refresh the cache.");
                    if let Err(err) = fs::remove_file(&user_path) {
                        eprintln!("Failed to remove corrupted cache file: {}", err);
                    }
                    e
                })?;
                Ok(user_detail)
            }
            Err(_) => {
                log::debug!("Cache miss: fetching user profile from LeetCode...");
                let user_detail = self.client.get_user_detail().await?;
                if let Ok(data) = serde_json::to_string(&user_detail) {
                    let _ = fs::write(&user_path, &data);
                }
                Ok(user_detail)
            }
        }
    }

    /// Returns the full problem list, enriched with topic tags.
    ///
    /// **Cache-aside strategy:**
    /// 1. Read `data.json` from disk.
    /// 2. If found, return it immediately and spawn a background task that
    ///    fetches a fresh list (problems + tags) and overwrites the cache.
    /// 3. If not found, block on both API calls, write the cache, then return.
    pub async fn list_problems(&self) -> crate::error::Result<Vec<ProblemSummary>> {
        let data_path = self.cache.problems_path();
        let user_path = self.cache.user_path();
        match fs::read_to_string(&data_path) {
            Ok(v) => {
                // Fetch data in the background and update data.json for next time
                let client_clone = self.client.clone();
                let data_path_bg = data_path.clone();
                let user_path_bg = user_path.clone();
                tokio::spawn(async move {
                    let res: Result<(), Box<dyn std::error::Error + Send + Sync>> = async {
                        let user_res = client_clone.get_user_detail();
                        let problems_res = client_clone.get_problem_list();
                        let tags_res = client_clone.get_topics_question_list();

                        let (user_detail, (mut problems, id_map), question_tags) =
                            tokio::try_join!(user_res, problems_res, tags_res)?;

                        let user_data = serde_json::to_string(&user_detail)?;
                        let _ = fs::write(&user_path_bg, user_data);

                        attach_topics(&mut problems, question_tags, &id_map);
                        let data = serde_json::to_string(&problems)?;
                        let _ = fs::write(&data_path_bg, data);
                        Ok(())
                    }
                    .await;

                    let _ = res;
                });

                let problems: Vec<ProblemSummary> = serde_json::from_str(&v).map_err(|e| {
                    eprintln!("Failed to parse problem list: {}", e);
                    eprintln!("Try running `leetrs tui` again to refresh the cache.");
                    if let Err(err) = fs::remove_file(&data_path) {
                        eprintln!("Failed to remove corrupted cache file: {}", err);
                    }
                    e
                })?;
                Ok(problems)
            }
            Err(_) => {
                log::debug!("Cache miss: fetching problem list and topic tags concurrently...");
                let ((mut problems, id_map), question_tags) = tokio::try_join!(
                    self.client.get_problem_list(),
                    self.client.get_topics_question_list()
                )?;
                attach_topics(&mut problems, question_tags, &id_map);
                if let Ok(data) = serde_json::to_string(&problems) {
                    let _ = fs::write(&data_path, &data);
                }
                Ok(problems)
            }
        }
    }
}

/// Merges topic tag names into each problem's `topics` list in O(P + T * Q) time
/// using a hash map lookup instead of a nested linear search.
///
/// LeetCode's `questionTopicTags` GraphQL response lists problems by their
/// internal database `questionId`. This function translates each internal ID
/// to the public `frontend_question_id` (matching `ProblemSummary.id`) before
/// attaching topics.
fn attach_topics(
    problems: &mut [ProblemSummary],
    question_tags: Vec<crate::models::QuestionTopics>,
    internal_to_frontend: &std::collections::HashMap<u64, u64>,
) {
    let mut problem_map: std::collections::HashMap<u64, &mut ProblemSummary> =
        problems.iter_mut().map(|p| (p.id, p)).collect();

    for tag in question_tags {
        for internal_id in tag.question_ids {
            // Problems where internal_id == frontend_id will always be found in
            // the map (the REST endpoint lists all problems). If a mapping is
            // missing we skip rather than guessing, to avoid attaching the wrong
            // topic to an unrelated problem.
            let Some(&frontend_id) = internal_to_frontend.get(&internal_id) else {
                continue;
            };

            if let Some(problem) = problem_map.get_mut(&frontend_id) {
                problem.topics.push(tag.name.clone());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{ProblemSummary, QuestionTopics};
    use std::collections::HashMap;
    use std::path::Path;

    #[test]
    fn attach_topics_maps_tags_to_matching_problems() {
        let mut problems = vec![
            ProblemSummary {
                id: 1,
                acceptance: 50.0,
                accepted: 100,
                difficulty: 1,
                slug: "two-sum".to_string(),
                status: None,
                submitted: 200,
                title: "Two Sum".to_string(),
                is_paid: false,
                topics: vec![],
            },
            ProblemSummary {
                id: 2,
                acceptance: 40.0,
                accepted: 80,
                difficulty: 2,
                slug: "add-two-numbers".to_string(),
                status: None,
                submitted: 200,
                title: "Add Two Numbers".to_string(),
                is_paid: false,
                topics: vec![],
            },
        ];

        let tags = vec![
            QuestionTopics {
                name: "Array".to_string(),
                id: "array".to_string(),
                slug: "array".to_string(),
                translated_name: None,
                question_ids: vec![1],
            },
            QuestionTopics {
                name: "Math".to_string(),
                id: "math".to_string(),
                slug: "math".to_string(),
                translated_name: None,
                question_ids: vec![2],
            },
            QuestionTopics {
                name: "Hash Table".to_string(),
                id: "hash-table".to_string(),
                slug: "hash-table".to_string(),
                translated_name: None,
                question_ids: vec![1],
            },
        ];

        // These problems have equal internal and frontend IDs — the common case
        // for older problems. The map must still be populated; we no longer fall
        // back to the raw internal ID when a mapping is absent.
        let id_map = HashMap::from([(1u64, 1u64), (2u64, 2u64)]);
        attach_topics(&mut problems, tags, &id_map);

        assert_eq!(problems[0].topics, vec!["Array", "Hash Table"]);
        assert_eq!(problems[1].topics, vec!["Math"]);
    }

    #[test]
    fn attach_topics_translates_unequal_internal_and_frontend_ids() {
        let mut problems = vec![
            ProblemSummary {
                id: 1550, // Public Frontend ID
                acceptance: 50.0,
                accepted: 100,
                difficulty: 1,
                slug: "three-consecutive-odds".to_string(),
                status: None,
                submitted: 200,
                title: "Three Consecutive Odds".to_string(),
                is_paid: false,
                topics: vec![],
            },
            ProblemSummary {
                id: 1677, // Another problem whose frontend ID equals 1677
                acceptance: 40.0,
                accepted: 80,
                difficulty: 2,
                slug: "products-worth-over-invoice".to_string(),
                status: None,
                submitted: 200,
                title: "Products' Worth Over Invoices".to_string(),
                is_paid: false,
                topics: vec![],
            },
        ];

        // Mapping: internal ID 1677 translates to public frontend ID 1550,
        // while internal ID 1800 translates to public frontend ID 1677.
        let mut id_map = HashMap::new();
        id_map.insert(1677, 1550);
        id_map.insert(1800, 1677);

        // Tags from GraphQL with internal question IDs
        let tags = vec![
            QuestionTopics {
                name: "Array".to_string(),
                id: "array".to_string(),
                slug: "array".to_string(),
                translated_name: None,
                question_ids: vec![1677],
            },
            QuestionTopics {
                name: "Database".to_string(),
                id: "database".to_string(),
                slug: "database".to_string(),
                translated_name: None,
                question_ids: vec![1800],
            },
        ];

        attach_topics(&mut problems, tags, &id_map);

        // Verify topic "Array" was attached to problem #1550 (not problem #1677)
        assert_eq!(problems[0].topics, vec!["Array"]);
        assert_eq!(problems[1].topics, vec!["Database"]);
    }

    use crate::models::{Question, QuestionSnippet};

    #[derive(Clone)]
    struct MockLeetCodeClient {
        question: Question,
    }

    impl LeetCodeApi for MockLeetCodeClient {
        async fn get_question_by_slug(&self, _title_slug: &str) -> crate::error::Result<Question> {
            Ok(self.question.clone())
        }

        async fn get_question_by_id(&self, _id: u64) -> crate::error::Result<Question> {
            Ok(self.question.clone())
        }

        async fn test_code(
            &self,
            _title_slug: &str,
            _question_id: &str,
            _lang: &str,
            _code: &str,
            _test_cases: &str,
        ) -> crate::error::Result<String> {
            unimplemented!()
        }

        async fn submit_code(
            &self,
            _title_slug: &str,
            _question_id: &str,
            _lang: &str,
            _code: &str,
        ) -> crate::error::Result<u64> {
            unimplemented!()
        }

        async fn check_test_submission(
            &self,
            _interpret_id: String,
        ) -> crate::error::Result<crate::models::TestSubmissionCheckResult> {
            unimplemented!()
        }

        async fn check_submission(
            &self,
            _submission_id: u64,
        ) -> crate::error::Result<crate::models::SubmissionCheckResult> {
            unimplemented!()
        }

        async fn get_user_detail(&self) -> crate::error::Result<crate::models::UserDetail> {
            Ok(crate::models::UserDetail {
                username: Some("test_user".to_string()),
                is_premium: Some(true),
                is_verified: true,
            })
        }

        async fn get_topics_question_list(
            &self,
        ) -> crate::error::Result<Vec<crate::models::QuestionTopics>> {
            Ok(vec![crate::models::QuestionTopics {
                name: "Array".to_string(),
                id: "array".to_string(),
                slug: "array".to_string(),
                translated_name: None,
                question_ids: vec![100],
            }])
        }

        async fn get_problem_list(
            &self,
        ) -> crate::error::Result<(
            Vec<crate::models::ProblemSummary>,
            std::collections::HashMap<u64, u64>,
        )> {
            let mut id_map = std::collections::HashMap::new();
            id_map.insert(100, 1);
            let problems = vec![crate::models::ProblemSummary {
                id: 1,
                title: "Two Sum".to_string(),
                slug: "two-sum".to_string(),
                difficulty: 1,
                accepted: 100,
                submitted: 200,
                acceptance: 0.5,
                status: Some("ac".to_string()),
                is_paid: false,
                topics: vec![],
            }];
            Ok((problems, id_map))
        }

        async fn get_daily_challenge(
            &self,
        ) -> crate::error::Result<crate::models::ActiveDailyQuestion> {
            unimplemented!()
        }
    }

    fn dummy_question() -> Question {
        Question {
            question_id: "1".to_string(),
            question_frontend_id: "1".to_string(),
            title_slug: "two-sum".to_string(),
            title: "Two Sum".to_string(),
            content: "<p>Content</p>".to_string(),
            example_test_cases: "".to_string(),
            code_snippets: vec![QuestionSnippet {
                lang_slug: "rust".to_string(),
                code: "// code".to_string(),
            }],
        }
    }

    #[tokio::test]
    async fn pick_by_id_does_not_overwrite_existing_solution() {
        let code_file = "test_pick_data_loss_problem.rs";
        let desc_file = "test_pick_data_loss_problem.md";
        let _ = fs::remove_file(code_file);
        let _ = fs::remove_file(desc_file);

        let user_code = "// My solved solution for problem 9999\nimpl Solution {}";
        fs::write(code_file, user_code).unwrap();

        let mock_question = Question {
            question_id: "9999".to_string(),
            question_frontend_id: "9999".to_string(),
            title_slug: "test-pick-data-loss-problem".to_string(),
            title: "Test Pick Data Loss Problem".to_string(),
            content: "<p>Problem content</p>".to_string(),
            example_test_cases: "".to_string(),
            code_snippets: vec![QuestionSnippet {
                lang_slug: "rust".to_string(),
                code: "// Empty starter code from leetcode".to_string(),
            }],
        };

        let picker = Picker::new(MockLeetCodeClient {
            question: mock_question,
        });

        let res = picker
            .pick(&Identifier::Number(9999), &Some(Language::Rust))
            .await;
        assert!(res.is_ok());

        let code_content = fs::read_to_string(code_file).unwrap();
        let _ = fs::remove_file(code_file);
        let _ = fs::remove_file(desc_file);

        assert_eq!(
            code_content, user_code,
            "Existing solution code must NOT be overwritten when picking by ID!"
        );
    }

    #[tokio::test]
    async fn pick_by_slug_keeps_existing_code_when_desc_is_missing() {
        let code_file = "test_missing_desc_problem.rs";
        let desc_file = "test_missing_desc_problem.md";
        let _ = fs::remove_file(code_file);
        let _ = fs::remove_file(desc_file);

        let user_code = "// My existing solution without description\nimpl Solution {}";
        fs::write(code_file, user_code).unwrap();

        let mock_question = Question {
            question_id: "8888".to_string(),
            question_frontend_id: "8888".to_string(),
            title_slug: "test-missing-desc-problem".to_string(),
            title: "Test Missing Desc Problem".to_string(),
            content: "<p>Problem content</p>".to_string(),
            example_test_cases: "".to_string(),
            code_snippets: vec![QuestionSnippet {
                lang_slug: "rust".to_string(),
                code: "// Empty starter code".to_string(),
            }],
        };

        let picker = Picker::new(MockLeetCodeClient {
            question: mock_question,
        });

        let res = picker
            .pick(
                &Identifier::String("test-missing-desc-problem".to_string()),
                &Some(Language::Rust),
            )
            .await;
        assert!(res.is_ok());

        let code_content = fs::read_to_string(code_file).unwrap();
        let desc_exists = Path::new(desc_file).exists();
        let _ = fs::remove_file(code_file);
        let _ = fs::remove_file(desc_file);

        assert_eq!(
            code_content, user_code,
            "Existing code must be preserved even if description was missing!"
        );
        assert!(desc_exists, "Missing description file should be created!");
    }

    #[tokio::test]
    async fn pick_by_id_fast_path_when_in_cache_skips_network() {
        let code_file = "test_fast_path_problem.rs";
        let desc_file = "test_fast_path_problem.md";
        let _ = fs::remove_file(code_file);
        let _ = fs::remove_file(desc_file);

        let user_code = "// Fast path user code";
        let user_desc = "# Fast path problem description";
        fs::write(code_file, user_code).unwrap();
        fs::write(desc_file, user_desc).unwrap();

        // Write cached problem entry
        let temp_dir = crate::cache::TempTestDir::new("leetrs-fast-path");
        let cache = CacheService::with_dir(temp_dir.path().to_path_buf());
        let cached_problems = vec![ProblemSummary {
            id: 7777,
            title: "Test Fast Path Problem".to_string(),
            slug: "test-fast-path-problem".to_string(),
            difficulty: 1,
            acceptance: 60.0,
            accepted: 100,
            submitted: 200,
            status: None,
            is_paid: false,
            topics: vec![],
        }];
        cache.write("data.json", &cached_problems).unwrap();

        // Mock client that panics if any network method is called
        #[derive(Clone)]
        struct PanickingMockClient;
        impl LeetCodeApi for PanickingMockClient {
            async fn get_question_by_slug(
                &self,
                _title_slug: &str,
            ) -> crate::error::Result<Question> {
                panic!("Network should not be called when both files exist and slug is cached!");
            }
            async fn get_question_by_id(&self, _id: u64) -> crate::error::Result<Question> {
                panic!("Network should not be called when both files exist and slug is cached!");
            }
            async fn test_code(
                &self,
                _: &str,
                _: &str,
                _: &str,
                _: &str,
                _: &str,
            ) -> crate::error::Result<String> {
                unimplemented!()
            }
            async fn submit_code(
                &self,
                _: &str,
                _: &str,
                _: &str,
                _: &str,
            ) -> crate::error::Result<u64> {
                unimplemented!()
            }
            async fn check_test_submission(
                &self,
                _: String,
            ) -> crate::error::Result<crate::models::TestSubmissionCheckResult> {
                unimplemented!()
            }
            async fn check_submission(
                &self,
                _: u64,
            ) -> crate::error::Result<crate::models::SubmissionCheckResult> {
                unimplemented!()
            }
            async fn get_user_detail(&self) -> crate::error::Result<crate::models::UserDetail> {
                unimplemented!()
            }
            async fn get_topics_question_list(
                &self,
            ) -> crate::error::Result<Vec<crate::models::QuestionTopics>> {
                unimplemented!()
            }
            async fn get_problem_list(
                &self,
            ) -> crate::error::Result<(
                Vec<crate::models::ProblemSummary>,
                std::collections::HashMap<u64, u64>,
            )> {
                unimplemented!()
            }
            async fn get_daily_challenge(
                &self,
            ) -> crate::error::Result<crate::models::ActiveDailyQuestion> {
                unimplemented!()
            }
        }

        let picker = Picker::with_cache(PanickingMockClient, cache);
        let res = picker
            .pick(&Identifier::Number(7777), &Some(Language::Rust))
            .await;

        let code_content = fs::read_to_string(code_file).unwrap();
        let desc_content = fs::read_to_string(desc_file).unwrap();

        // Clean up
        let _ = fs::remove_file(code_file);
        let _ = fs::remove_file(desc_file);

        assert!(res.is_ok());
        assert_eq!(code_content, user_code);
        assert_eq!(desc_content, user_desc);
    }

    #[tokio::test]
    async fn test_picker_get_user_data_cold_and_warm() {
        let temp_dir = crate::cache::TempTestDir::new("leetrs-picker-user");
        let cache = CacheService::with_dir(temp_dir.path().to_path_buf());
        let user_path = cache.user_path();

        let picker = Picker::with_cache(
            MockLeetCodeClient {
                question: dummy_question(),
            },
            cache,
        );

        // 1. Cold cache fetch
        let user = picker.get_user_data().await.unwrap();
        assert_eq!(user.username.as_deref(), Some("test_user"));
        assert!(user_path.exists());

        // 2. Warm cache read
        let cached_user = picker.get_user_data().await.unwrap();
        assert_eq!(cached_user.username.as_deref(), Some("test_user"));

        // Allow background refresh task spawned during warm cache read to run
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    }

    #[tokio::test]
    async fn test_picker_list_problems_cold_and_warm() {
        let temp_dir = crate::cache::TempTestDir::new("leetrs-picker-problems");
        let cache = CacheService::with_dir(temp_dir.path().to_path_buf());
        let data_path = cache.problems_path();

        let picker = Picker::with_cache(
            MockLeetCodeClient {
                question: dummy_question(),
            },
            cache,
        );

        // 1. Cold cache fetch
        let problems = picker.list_problems().await.unwrap();
        assert_eq!(problems.len(), 1);
        assert_eq!(problems[0].id, 1);
        assert_eq!(problems[0].topics, vec!["Array".to_string()]);
        assert!(data_path.exists());

        // 2. Warm cache read
        let cached_problems = picker.list_problems().await.unwrap();
        assert_eq!(cached_problems.len(), 1);
        assert_eq!(cached_problems[0].id, 1);
        assert_eq!(cached_problems[0].topics, vec!["Array".to_string()]);

        // Allow background refresh task spawned during warm cache read to run
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    }
}
