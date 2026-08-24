//! Problem picker — fetches questions, writes local files, and drives submission.
//!
//! [`Picker`] is the main orchestrator used by both the CLI commands (`pick`,
//! `submit`, `test`) and the TUI. It wraps [`LeetCodeClient`] and adds local
//! file I/O and a disk cache for the problem list.
use crate::cache::CacheService;
use crate::config::CONFIG;
use crate::error::EngineError;
use crate::format::format_result;
use crate::models::{Identifier, ProblemSummary, UserDetail};
use crate::services::submission::SubmissionService;
use crate::{client::LeetCodeClient, models::Language};
use std::fs;
use std::path::Path;

/// Orchestrates problem fetching, file generation, submission, and caching.
///
/// Constructed once per command invocation and shared across async tasks via
/// clone (the inner [`LeetCodeClient`] is `Clone`).
#[derive(Clone)]
pub struct Picker {
    pub client: LeetCodeClient,
}

impl Picker {
    pub fn new(client: LeetCodeClient) -> Self {
        Picker { client }
    }

    /// Resolves a problem by [`Identifier`], writes the Markdown description
    /// and language-specific code stub to disk, and returns their paths.
    ///
    /// If both files already exist on disk (slug-based match) they are returned
    /// immediately without hitting the network.
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

        //TODO: If language is specified, must open that file
        // else open the file with matching slug.
        if let Identifier::String(ident) = identifier {
            let snake_slug = ident.replace("-", "_");
            let code_filename = format!("{}.{}", snake_slug, language.code_extension());
            let desc_filename = format!("{}.md", snake_slug);
            if Path::new(&code_filename).exists() && Path::new(&desc_filename).exists() {
                return Ok((code_filename, desc_filename));
            }
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

        //  determine filenames (converting kebab-case to snake_case)
        let snake_slug = question.title_slug.replace("-", "_");
        let code_filename = format!("{}.{}", snake_slug, language.code_extension());
        let desc_filename = format!("{}.md", snake_slug);

        let meta = format!(
            "{} id={} slug={} lang={}",
            language.meta_comment_prefix(),
            question.question_id,
            question.title_slug,
            language.to_lang_slug()
        );

        if let Err(e) = fs::write(&code_filename, format!("{}\n\n{}", meta, snippet.code)) {
            eprintln!("❌ failed to write code file: {}", e);
            return Err(EngineError::System);
        }
        if let Err(e) = fs::write(&desc_filename, md_content) {
            eprintln!("❌ failed to write description file: {}", e);
            return Err(EngineError::System);
        }
        println!("✅ files generated successfully.");

        Ok((code_filename, desc_filename))
    }

    /// Runs the solution file against the problem's built-in example test cases
    /// and prints the result, but **does not** record it as an official submission.
    pub async fn test_submit(&self, file: &str) {
        let service = SubmissionService::new(self.client.clone());
        match service.submit_or_test(file, true).await {
            Ok(result) => format_result(&result),
            Err(e) => eprintln!("❌ {}", e),
        }
    }

    /// Submits the solution file to LeetCode for full judging and prints the
    /// verdict, test-case counts, and performance percentiles.
    pub async fn submit(&self, file: &str) {
        let service = SubmissionService::new(self.client.clone());
        match service.submit_or_test(file, false).await {
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
        let cache = CacheService::new();
        let user_path = cache.user_path();
        let data = match fs::read_to_string(&user_path) {
            Ok(v) => {
                let client = self.client.clone();
                let user_path_bg = user_path.clone();
                tokio::spawn(async move {
                    let result: Result<(), Box<dyn std::error::Error>> = async {
                        let user_detail = client.get_user_detail().await?;
                        let data = serde_json::to_string(&user_detail)?;
                        let _ = fs::write(&user_path_bg, &data);
                        Ok(())
                    }
                    .await;

                    let _ = result;
                });
                v
            }
            Err(_) => {
                let user_detail = self.client.get_user_detail().await?;
                let data = serde_json::to_string(&user_detail)?;
                let _ = fs::write(&user_path, &data);
                data
            }
        };
        let user_detail: UserDetail = serde_json::from_str(&data).map_err(|e| {
            eprintln!("Failed to parse user details: {}", e);
            eprintln!("Try running `leetrs tui` again to refresh the cache.");
            if let Err(err) = fs::remove_file(&user_path) {
                eprintln!("Failed to remove corrupted cache file: {}", err);
            }
            e
        })?;
        Ok(user_detail)
    }

    /// Returns the full problem list, enriched with topic tags.
    ///
    /// **Cache-aside strategy:**
    /// 1. Read `data.json` from disk.
    /// 2. If found, return it immediately and spawn a background task that
    ///    fetches a fresh list (problems + tags) and overwrites the cache.
    /// 3. If not found, block on both API calls, write the cache, then return.
    pub async fn list_problems(&self) -> crate::error::Result<Vec<ProblemSummary>> {
        let cache = CacheService::new();
        let data_path = cache.problems_path();
        let user_path = cache.user_path();
        let data = match fs::read_to_string(&data_path) {
            Ok(v) => {
                // Fetch data in the background and update data.json for next time
                let client_clone = self.client.clone();
                let data_path_bg = data_path.clone();
                let user_path_bg = user_path.clone();
                tokio::spawn(async move {
                    let res: Result<(), Box<dyn std::error::Error + Send + Sync>> = async {
                        let user_detail = client_clone.get_user_detail().await?;
                        let data = serde_json::to_string(&user_detail)?;
                        let _ = fs::write(&user_path_bg, data);
                        let mut problems = client_clone.get_problem_list().await?;
                        let question_tags = client_clone.get_topics_question_list().await?;
                        attach_topics(&mut problems, question_tags);
                        let data = serde_json::to_string(&problems)?;
                        let _ = fs::write(&data_path_bg, data);
                        Ok(())
                    }
                    .await;

                    let _ = res;
                });
                v
            }
            Err(_) => {
                let mut problems = self.client.get_problem_list().await?;
                let question_tags = self.client.get_topics_question_list().await?;
                attach_topics(&mut problems, question_tags);
                let data = serde_json::to_string(&problems)?;
                let _ = fs::write(&data_path, &data);
                data
            }
        };
        let problems: Vec<ProblemSummary> = serde_json::from_str(&data).map_err(|e| {
            eprintln!("Failed to parse problem list: {}", e);
            eprintln!("Try running `leetrs tui` again to refresh the cache.");
            if let Err(err) = fs::remove_file(&data_path) {
                eprintln!("Failed to remove corrupted cache file: {}", err);
            }
            e
        })?;
        Ok(problems)
    }
}

/// Merges topic tag names into each problem's `topics` list in O(P + T * Q) time
/// using a hash map lookup instead of a nested linear search.
fn attach_topics(
    problems: &mut [ProblemSummary],
    question_tags: Vec<crate::models::QuestionTopics>,
) {
    use std::collections::HashMap;

    let mut problem_map: HashMap<u64, &mut ProblemSummary> =
        problems.iter_mut().map(|p| (p.id, p)).collect();

    for tag in question_tags {
        for q_id in tag.question_ids {
            if let Some(problem) = problem_map.get_mut(&q_id) {
                problem.topics.push(tag.name.clone());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{ProblemSummary, QuestionTopics};

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

        attach_topics(&mut problems, tags);

        assert_eq!(problems[0].topics, vec!["Array", "Hash Table"]);
        assert_eq!(problems[1].topics, vec!["Math"]);
    }
}
