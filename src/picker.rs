//! Problem picker — fetches questions, writes local files, and drives submission.
//!
//! [`Picker`] is the main orchestrator used by both the CLI commands (`pick`,
//! `submit`, `test`) and the TUI. It wraps [`LeetCodeClient`] and adds local
//! file I/O and a disk cache for the problem list.
use crate::error::EngineError;
use crate::models::{Identifier, ProblemSummary, UserDetail};
use crate::{client::LeetCodeClient, models::Language};
use std::path::Path;
use std::{fs, process};

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

    /// Returns the path to `data.json` (the cached problem list) inside the
    /// OS-standard data directory, creating it if it doesn't already exist.
    pub fn get_data_path() -> String {
        let project_dirs = directories::ProjectDirs::from("com", "shadowmkj", "leetrs").unwrap();
        let data_dir = project_dirs.data_dir();
        if !data_dir.exists()
            && let Err(e) = fs::create_dir_all(data_dir)
        {
            eprintln!("❌ Failed to create data directory: {}", e);
            process::exit(1);
        }
        data_dir.join("data.json").to_str().unwrap().to_string()
    }

    /// Returns the path to `user.json` (the cached user profile) inside the
    /// OS-standard data directory, creating it if it doesn't already exist.
    pub fn get_user_data_path() -> String {
        let project_dirs = directories::ProjectDirs::from("com", "shadowmkj", "leetrs").unwrap();
        let data_dir = project_dirs.data_dir();
        if !data_dir.exists()
            && let Err(e) = fs::create_dir_all(data_dir)
        {
            eprintln!("❌ Failed to create data directory: {}", e);
            process::exit(1);
        }
        data_dir.join("user.json").to_str().unwrap().to_string()
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
                println!("🔤 No language specified, defaulting to Python.");
                Language::Python
            }
        };

        //TODO: If language is specified, must open that file
        // else open the file with matching slug.
        if let Identifier::String(ident) = identifier {
            let snake_slug = ident.replace("-", "_");
            let code_filename = match language {
                Language::Python | Language::Pandas => format!("{}.py", snake_slug),
                Language::Rust => format!("{}.rs", snake_slug),
                Language::Mysql | Language::Postgres => format!("{}.sql", snake_slug),
            };
            let desc_filename = format!("{}.md", snake_slug);
            if Path::new(&code_filename).exists() && Path::new(&desc_filename).exists() {
                return Ok((code_filename, desc_filename));
            }
        }

        let question = match identifier {
            Identifier::Number(num) => {
                println!("🔍 Fetching problem ID: {}...", num);
                self.client.get_question_by_id(*num).await.unwrap()
            }
            Identifier::String(identifier) => {
                println!("🔍 Fetching problem: {}...", identifier);
                self.client.get_question_by_slug(identifier).await.unwrap()
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
                let snippet = question
                    .code_snippets
                    .first()
                    .expect("Leetcode problems must have atleast one snippet");
                language = Language::from(snippet.lang_slug.clone());
                snippet
            }
        };

        //  determine filenames (converting kebab-case to snake_case)
        let snake_slug = question.title_slug.replace("-", "_");
        let code_filename = match language {
            Language::Rust => format!("{}.rs", snake_slug),
            Language::Python | Language::Pandas => format!("{}.py", snake_slug),
            Language::Mysql | Language::Postgres => format!("{}.sql", snake_slug),
        };
        let desc_filename = format!("{}.md", snake_slug);

        let meta = match language {
            Language::Python | Language::Pandas => {
                format!(
                    "# id={} slug={} lang={}",
                    question.question_id,
                    question.title_slug,
                    language.to_lang_slug()
                )
            }
            Language::Rust => format!(
                "// id={} slug={} lang={}",
                question.question_id,
                question.title_slug,
                language.to_lang_slug()
            ),
            Language::Mysql => format!(
                "# id={} slug={} lang={}",
                question.question_id,
                question.title_slug,
                language.to_lang_slug()
            ),
            Language::Postgres => format!(
                "-- id={} slug={} lang={}",
                question.question_id,
                question.title_slug,
                language.to_lang_slug()
            ),
        };

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
    pub async fn test_submit(&self, file: &String) {
        let code = match std::fs::read_to_string(file) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("❌ Failed to read file '{}': {}", file, e);
                return;
            }
        };

        // Extract the slug from the filename (e.g., "two_sum.rs" -> "two-sum")
        let path = std::path::Path::new(&file);
        let file_stem = path
            .file_stem()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();
        let slug = file_stem.replace("_", "-");
        println!("🔍 Resolving ID for '{}'...", slug);
        let language = Language::from_extension(
            path.extension()
                .and_then(|s| s.to_str())
                .unwrap_or_default(),
        );

        let question = match self.client.get_question_by_slug(&slug).await {
            Ok(q) => q,
            Err(e) => {
                eprintln!(
                    "❌ Failed to fetch question ID. Does the filename match the problem slug? Error: {}",
                    e
                );
                return;
            }
        };

        println!("🚀 Submitting {}...", file);
        let interpret_id = match self
            .client
            .test_code(
                &slug,
                &question.question_id,
                language.to_lang_slug(),
                &code,
                &question.example_test_cases,
            )
            .await
        {
            Ok(id) => id,
            Err(e) => {
                eprintln!("❌ Test Submission failed: {}", e);
                return;
            }
        };

        println!("⏳ Code queued. Waiting for execution results...");
        let result = match self.client.check_test_submission(interpret_id).await {
            Ok(r) => r,
            Err(e) => {
                eprintln!("❌ Failed to check test submission status: {}", e);
                return;
            }
        };

        println!("\n==================================================");

        let status = result.correct_answer.unwrap_or(false);

        if status {
            println!("  ✅ All test cases passed");
        } else {
            println!("  ❌ Test Failed");
        }

        println!("==================================================\n");

        if let (Some(correct), Some(total)) = (result.total_correct, result.total_testcases) {
            println!("🧪 Testcases: {} / {} passed", correct, total);
        }

        let status_msg = result.status_msg.unwrap_or("Unknown".to_string());
        if status && status_msg == "Accepted" {
            if let Some(runtime) = result.status_runtime {
                println!("⏱️ Runtime: {}", runtime);
            }
            if let Some(memory) = result.status_memory {
                println!("💾 Memory: {}", memory);
            }
            if let Some(memory_percentile) = result.memory_percentile {
                println!("📝 Memory Percentile: {:.2}%", memory_percentile);
            }
            if let Some(runtime_percentile) = result.runtime_percentile {
                println!("⏰ Runtime Percentile: {:.2}%", runtime_percentile);
            }
        } else if status_msg == "Accepted" {
            if let (Some(code_answers), Some(expected)) = (
                result.code_answer.as_ref(),
                result.expected_code_answer.as_ref(),
            ) {
                println!("Expected");
                println!("{}", expected.join("\t"));
                println!("Output");
                println!("{}", code_answers.join("\t"));
            }
        } else if status_msg == "Runtime Error"
            && let Some(runtime_error) = result.full_runtime_error
        {
            println!("❌ Error\n{}", runtime_error);
        }
    }

    /// Submits the solution file to LeetCode for full judging and prints the
    /// verdict, test-case counts, and performance percentiles.
    pub async fn submit(&self, file: &String) {
        let code = match std::fs::read_to_string(file) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("❌ Failed to read file '{}': {}", file, e);
                return;
            }
        };

        // Extract the slug from the filename (e.g., "two_sum.rs" -> "two-sum")
        let path = std::path::Path::new(&file);
        let file_stem = path
            .file_stem()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();
        let slug = file_stem.replace("_", "-");
        println!("🔍 Resolving ID for '{}'...", slug);
        let language = Language::from_extension(
            path.extension()
                .and_then(|s| s.to_str())
                .unwrap_or_default(),
        );

        let question = match self.client.get_question_by_slug(&slug).await {
            Ok(q) => q,
            Err(e) => {
                eprintln!(
                    "❌ Failed to fetch question ID. Does the filename match the problem slug? Error: {}",
                    e
                );
                return;
            }
        };

        println!("🚀 Submitting {}...", file);
        let submission_id = match self
            .client
            .submit_code(&slug, &question.question_id, language.to_lang_slug(), &code)
            .await
        {
            Ok(id) => id,
            Err(e) => {
                eprintln!("❌ Submission failed: {}", e);
                return;
            }
        };

        println!("⏳ Code queued. Waiting for execution results...");
        let result = match self.client.check_submission(submission_id).await {
            Ok(r) => r,
            Err(e) => {
                eprintln!("❌ Failed to check submission status: {}", e);
                return;
            }
        };

        println!("\n==================================================");

        let status = result.status_msg.unwrap_or_else(|| "Unknown".to_string());

        if status == "Accepted" {
            println!("  ✅ {}", status);
        } else {
            println!("  ❌ {}", status);
        }

        println!("==================================================\n");

        if let (Some(correct), Some(total)) = (result.total_correct, result.total_testcases) {
            println!("🧪 Testcases: {} / {} passed", correct, total);
        }

        if status == "Accepted" {
            if let Some(runtime) = result.status_runtime {
                println!("⏱️ Runtime: {}", runtime);
            }
            if let Some(memory) = result.status_memory {
                println!("💾 Memory: {}", memory);
            }
            if let Some(memory_percentile) = result.memory_percentile {
                println!("📝 Memory Percentile: {:.2}%", memory_percentile);
            }
            if let Some(runtime_percentile) = result.runtime_percentile {
                println!("⏰ Runtime Percentile: {:.2}%", runtime_percentile);
            }
        } else if status == "Compile Error" {
            if let Some(err_msg) = result.compile_error {
                println!("💥 Compiler Output:\n{}", err_msg);
            }
        } else if status == "Wrong Answer" {
            if let Some(input) = result.input {
                let parts = input.split("\n");
                print!("INPUT: ");
                for part in parts {
                    print!("{}\t", part);
                }
                println!();
            }
            if let Some(expected) = result.expected_output
                && let Some(output) = result.code_output
            {
                println!("Expected: {}\nOutput: {}", expected, output);
            }
        } else if status == "Runtime Error"
            && let Some(runtime_error) = result.full_runtime_error
        {
            println!("❌ Error\n{}", runtime_error);
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
        let data = match fs::read_to_string(Picker::get_user_data_path()) {
            Ok(v) => {
                let client = self.client.clone();
                tokio::spawn(async move {
                    let result: Result<(), Box<dyn std::error::Error>> = async {
                        let user_detail = client.get_user_detail().await?;
                        let data = serde_json::to_string(&user_detail)?;
                        let _ = fs::write(Picker::get_user_data_path(), &data);
                        Ok(())
                    }
                    .await;

                    // Handle any errors that occurred in the background task
                    if let Err(e) = result {
                        eprintln!("Failed to fetch/save user data in background: {}", e);
                    }
                });
                v
            }
            Err(_) => {
                let user_detail = self.client.get_user_detail().await?;
                let data = serde_json::to_string(&user_detail)?;
                let _ = fs::write(Picker::get_user_data_path(), &data);
                data
            }
        };
        let user_detail: UserDetail = serde_json::from_str(&data).map_err(|e| {
            eprintln!("Failed to parse user details: {}", e);
            eprintln!("Try running `leetrs tui` again to refresh the cache.");
            // Remove the data file since it's corrupted, so the next run will fetch fresh data
            // from the API
            if let Err(err) = fs::remove_file(Picker::get_user_data_path()) {
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
    pub async fn list_problems(&self) -> anyhow::Result<Vec<ProblemSummary>> {
        // let user_detail = self
        //     .client
        //     .get_user_detail()
        //     .await
        //     .expect("Failed to retrieve user details");
        // let data =
        //     serde_json::to_string(&user_detail).expect("Failed to serialize user_detail list");
        // fs::write(Picker::get_user_data_path(), data)
        //     .expect("Unable to write user data json to file");
        let data = match fs::read_to_string(Picker::get_data_path()) {
            Ok(v) => {
                // Fetch data in the background and update data.json for next time
                let client_clone = self.client.clone();
                tokio::spawn(async move {
                    let user_detail = client_clone
                        .get_user_detail()
                        .await
                        .expect("Failed to retrieve user details");
                    let data = serde_json::to_string(&user_detail)
                        .expect("Failed to serialize user_detail list");
                    fs::write(Picker::get_user_data_path(), data)
                        .expect("Unable to write user data json to file");
                    let mut problems = client_clone
                        .get_problem_list()
                        .await
                        .expect("Failed to fetch problem list");
                    let question_tags = client_clone
                        .get_topics_question_list()
                        .await
                        .expect("Failed to fetch topics list");
                    for question_tag in question_tags {
                        question_tag.question_ids.iter().for_each(|question_id| {
                            if let Some(problem) =
                                problems.iter_mut().find(|p| p.id == *question_id)
                            {
                                problem.topics.push(question_tag.name.clone());
                            }
                        });
                    }
                    let data =
                        serde_json::to_string(&problems).expect("Failed to serialize problem list");
                    fs::write(Picker::get_data_path(), data).expect("Unable to write json to file");
                });
                v
            }
            Err(_) => {
                let mut problems = self
                    .client
                    .get_problem_list()
                    .await
                    .expect("Failed to fetch problem list");
                let question_tags = self
                    .client
                    .get_topics_question_list()
                    .await
                    .expect("Failed to fetch topics list");
                for question_tag in question_tags {
                    question_tag.question_ids.iter().for_each(|question_id| {
                        if let Some(problem) = problems.iter_mut().find(|p| p.id == *question_id) {
                            problem.topics.push(question_tag.name.clone());
                        }
                    });
                }
                let data =
                    serde_json::to_string(&problems).expect("Failed to serialize problem list");
                fs::write(Picker::get_data_path(), &data).expect("Unable to write json to file");
                data
            }
        };
        let problems: Vec<ProblemSummary> = serde_json::from_str(&data).map_err(|e| {
            eprintln!("Failed to parse problem list: {}", e);
            eprintln!("Try running `leetrs tui` again to refresh the cache.");
            // Remove the data file since it's corrupted, so the next run will fetch fresh data
            // from the API
            if let Err(err) = fs::remove_file(Picker::get_data_path()) {
                eprintln!("Failed to remove corrupted cache file: {}", err);
            }
            e
        })?;
        Ok(problems)
    }
}

#[cfg(test)]
mod tests {
    use crate::models::Language;

    // -----------------------------------------------------------------------
    // Slug / filename derivation (mirrors logic in Picker::pick)
    // -----------------------------------------------------------------------

    /// Converts a kebab-case LeetCode slug to the snake_case form used for filenames.
    fn slug_to_snake(slug: &str) -> String {
        slug.replace('-', "_")
    }

    /// Converts a snake_case filename stem back to the kebab-case API slug.
    fn stem_to_slug(stem: &str) -> String {
        stem.replace('_', "-")
    }

    #[test]
    fn slug_to_snake_single_word() {
        assert_eq!(slug_to_snake("fizzbuzz"), "fizzbuzz");
    }

    #[test]
    fn slug_to_snake_two_words() {
        assert_eq!(slug_to_snake("two-sum"), "two_sum");
    }

    #[test]
    fn slug_to_snake_multiple_hyphens() {
        assert_eq!(
            slug_to_snake("longest-palindromic-substring"),
            "longest_palindromic_substring"
        );
    }

    #[test]
    fn slug_to_snake_empty_string() {
        assert_eq!(slug_to_snake(""), "");
    }

    #[test]
    fn stem_to_slug_roundtrip() {
        let slug = "trapping-rain-water";
        assert_eq!(stem_to_slug(&slug_to_snake(slug)), slug);
    }

    #[test]
    fn stem_to_slug_roundtrip_single_word() {
        let slug = "median";
        assert_eq!(stem_to_slug(&slug_to_snake(slug)), slug);
    }

    // -----------------------------------------------------------------------
    // Code filename derivation per language
    // -----------------------------------------------------------------------

    fn code_filename(snake_slug: &str, lang: &Language) -> String {
        match lang {
            Language::Rust => format!("{}.rs", snake_slug),
            Language::Python | Language::Pandas => format!("{}.py", snake_slug),
            Language::Mysql | Language::Postgres => format!("{}.sql", snake_slug),
        }
    }

    #[test]
    fn code_filename_rust() {
        assert_eq!(code_filename("two_sum", &Language::Rust), "two_sum.rs");
    }

    #[test]
    fn code_filename_python() {
        assert_eq!(code_filename("two_sum", &Language::Python), "two_sum.py");
    }

    #[test]
    fn code_filename_pandas_uses_py_extension() {
        // Pandas is a Python library — the file extension must also be .py.
        assert_eq!(
            code_filename("group_sold_products", &Language::Pandas),
            "group_sold_products.py"
        );
    }

    #[test]
    fn code_filename_mysql() {
        assert_eq!(
            code_filename("combine_two_tables", &Language::Mysql),
            "combine_two_tables.sql"
        );
    }

    #[test]
    fn code_filename_postgres_uses_sql_extension() {
        assert_eq!(
            code_filename("combine_two_tables", &Language::Postgres),
            "combine_two_tables.sql"
        );
    }

    #[test]
    fn desc_filename_always_md() {
        let desc = format!("{}.md", slug_to_snake("longest-palindromic-substring"));
        assert_eq!(desc, "longest_palindromic_substring.md");
    }

    // -----------------------------------------------------------------------
    // Metadata comment prefix per language
    // -----------------------------------------------------------------------

    fn meta_prefix(lang: &Language) -> &'static str {
        match lang {
            Language::Python | Language::Pandas | Language::Mysql => "#",
            Language::Rust => "//",
            Language::Postgres => "--",
        }
    }

    #[test]
    fn meta_prefix_python_is_hash() {
        assert_eq!(meta_prefix(&Language::Python), "#");
    }

    #[test]
    fn meta_prefix_pandas_is_hash() {
        assert_eq!(meta_prefix(&Language::Pandas), "#");
    }

    #[test]
    fn meta_prefix_rust_is_double_slash() {
        assert_eq!(meta_prefix(&Language::Rust), "//");
    }

    #[test]
    fn meta_prefix_mysql_is_hash() {
        assert_eq!(meta_prefix(&Language::Mysql), "#");
    }

    #[test]
    fn meta_prefix_postgres_is_double_dash() {
        assert_eq!(meta_prefix(&Language::Postgres), "--");
    }

    #[test]
    fn metadata_line_format_rust() {
        let lang = Language::Rust;
        let line = format!(
            "{} id={} slug={} lang={}",
            meta_prefix(&lang),
            1,
            "two-sum",
            lang.to_lang_slug()
        );
        assert_eq!(line, "// id=1 slug=two-sum lang=rust");
    }

    #[test]
    fn metadata_line_format_python() {
        let lang = Language::Python;
        let line = format!(
            "{} id={} slug={} lang={}",
            meta_prefix(&lang),
            42,
            "climbing-stairs",
            lang.to_lang_slug()
        );
        assert_eq!(line, "# id=42 slug=climbing-stairs lang=python3");
    }

    #[test]
    fn metadata_line_format_postgres() {
        let lang = Language::Postgres;
        let line = format!(
            "{} id={} slug={} lang={}",
            meta_prefix(&lang),
            175,
            "combine-two-tables",
            lang.to_lang_slug()
        );
        assert_eq!(line, "-- id=175 slug=combine-two-tables lang=postgresql");
    }

    // -----------------------------------------------------------------------
    // Acceptance ratio edge cases (mirrors get_problem_list arithmetic)
    // -----------------------------------------------------------------------

    fn acceptance_ratio(accepted: u64, submitted: u64) -> f64 {
        accepted as f64 / submitted as f64
    }

    #[test]
    fn acceptance_ratio_normal() {
        let r = acceptance_ratio(500, 1000);
        assert!((r - 0.5).abs() < 1e-9);
    }

    #[test]
    fn acceptance_ratio_perfect() {
        let r = acceptance_ratio(1000, 1000);
        assert!((r - 1.0).abs() < 1e-9);
    }

    #[test]
    fn acceptance_ratio_zero_submissions_is_nan() {
        // 0/0 in floating point is NaN. This is what the current production code
        // produces; asserting it documents the known behavior explicitly.
        let r = acceptance_ratio(0, 0);
        assert!(r.is_nan(), "expected NaN for 0/0, got {r}");
    }

    #[test]
    fn acceptance_ratio_zero_accepted() {
        let r = acceptance_ratio(0, 100);
        assert!((r - 0.0).abs() < 1e-9);
    }
}
