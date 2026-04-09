use std::fs;

use crate::error::{EngineError, Result};
use crate::{client::LeetCodeClient, models::Language};

pub struct Picker {
    pub client: LeetCodeClient,
}

impl Picker {
    pub fn new(client: LeetCodeClient) -> Self {
        Picker { client }
    }

    pub async fn pick(
        &self,
        identifier: &String,
        language: &Option<Language>,
    ) -> Result<(String, String)> {
        let language = match language {
            Some(lang) => lang,
            None => {
                println!("🔤 No language specified, defaulting to Rust.");
                &Language::Rust
            }
        };

        let question = if identifier.chars().all(char::is_numeric) {
            let id: u64 = identifier.parse().unwrap();
            println!("🔍 Fetching problem ID: {}...", id);
            self.client.get_question_by_id(id, &language).await.unwrap()
        } else {
            println!("🔍 Fetching problem: {}...", identifier);
            self.client
                .get_question_by_slug(&identifier, &language)
                .await
                .unwrap()
        };

        // Convert LeetCode's raw HTML into wrapped terminal text (80 columns wide)
        let formatted_content = html2md::parse_html(&question.content);
        let md_content = format!("# {}\n\n{}", question.title, formatted_content);
        // 2. determine filenames (converting kebab-case to snake_case)
        let snake_slug = question.title_slug.replace("-", "_");
        let code_filename = match language {
            Language::Rust => format!("{}.rs", snake_slug),
            Language::Python => format!("{}.py", snake_slug),
        };
        let desc_filename = format!("{}.md", snake_slug);

        // 3. write both files to disk
        let snippet = question
            .code_snippets
            .into_iter()
            .find(|s| s.lang_slug == language.to_lang_slug());

        let meta = match language {
            Language::Python => {
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
        };

        if let Some(snippet) = snippet {
            if let Err(e) = fs::write(&code_filename, format!("{}\n\n{}", meta, snippet.code)) {
                eprintln!("❌ failed to write code file: {}", e);
                return Err(EngineError::System);
            }
            if let Err(e) = fs::write(&desc_filename, md_content) {
                eprintln!("❌ failed to write description file: {}", e);
                return Err(EngineError::System);
            }
            println!("✅ files generated successfully.");
        } else {
            eprintln!(
                "⚠️ no {} boilerplate found for this problem.",
                language.to_lang_slug()
            );
            return Err(EngineError::System);
        }

        return Ok((code_filename, desc_filename));
    }
}
