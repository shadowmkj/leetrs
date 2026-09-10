//! Data models shared across the crate.
//!
//! All types here are either serialised into HTTP request bodies or deserialised
//! from API responses. Serde field renames mirror LeetCode's camelCase JSON keys.
//!
//! # Module layout
//!
//! | Module | Contents |
//! |--------|----------|
//! | [`language`] | [`Language`] enum and [`Identifier`] |
//! | [`problem`] | [`ProblemSummary`], [`Question`], [`UserDetail`], [`GraphQLQuery`] |
//! | [`submission`] | Payload/response types for submit and test-run |

pub mod language;
pub mod problem;
pub mod submission;

// Re-export everything at the models level to keep all existing `use` paths
// working without any changes in other modules.
pub use language::{Identifier, Language};
pub use problem::{
    GraphQLQuery, ProblemSummary, Question, QuestionSnippet, QuestionTopics, Topic, UserDetail,
};
pub use submission::{
    SubmissionCheckResult, SubmitPayload, SubmitResponse, TestPayload, TestSubmissionCheckResult,
    TestSubmitResponse,
};

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    // -----------------------------------------------------------------------
    // Language::to_lang_slug
    // -----------------------------------------------------------------------

    #[test]
    fn language_slug_and_extension_mappings() {
        let cases = [
            (Language::Python, "python3", "py", "#"),
            (Language::Rust, "rust", "rs", "//"),
            (Language::Pandas, "pythondata", "py", "#"),
            (Language::Mysql, "mysql", "sql", "#"),
            (Language::Postgres, "postgresql", "sql", "--"),
        ];

        for (lang, slug, ext, prefix) in cases {
            assert_eq!(lang.to_lang_slug(), slug);
            assert_eq!(lang.code_extension(), ext);
            assert_eq!(lang.meta_comment_prefix(), prefix);
            let parsed = Language::from(slug.to_string());
            assert_eq!(parsed.to_lang_slug(), slug);
        }

        // Unknown slugs and extensions fall back to Mysql safely
        assert_eq!(
            Language::from("javascript".to_string()).to_lang_slug(),
            "mysql"
        );
        assert_eq!(Language::from("".to_string()).to_lang_slug(), "mysql");
        assert_eq!(Language::from_extension("js").to_lang_slug(), "mysql");
        assert_eq!(Language::from_extension("").to_lang_slug(), "mysql");
    }

    #[test]
    fn user_detail_deserializes_camel_case_json() {
        let raw = r#"{"username":"alice","isPremium":true,"isVerified":false}"#;
        let ud: UserDetail = serde_json::from_str(raw).expect("deserialization failed");
        assert_eq!(ud.username.as_deref(), Some("alice"));
        assert_eq!(ud.is_premium, Some(true));
        assert!(!ud.is_verified);
    }

    #[test]
    fn user_detail_null_username_and_premium() {
        let raw = r#"{"username":null,"isPremium":null,"isVerified":true}"#;
        let ud: UserDetail = serde_json::from_str(raw).expect("deserialization failed");
        assert!(ud.username.is_none());
        assert!(ud.is_premium.is_none());
        assert!(ud.is_verified);
    }

    #[test]
    fn test_submission_check_result_minimal_deserialize() {
        let raw = r#"{"state":"PENDING"}"#;
        let r: TestSubmissionCheckResult =
            serde_json::from_str(raw).expect("should accept minimal structure");
        assert_eq!(r.state, "PENDING");
        assert!(r.correct_answer.is_none());
        assert!(r.total_correct.is_none());
    }

    #[test]
    fn test_submission_check_result_full_deserialize() {
        let raw = r#"{
            "state": "SUCCESS",
            "correct_answer": true,
            "total_correct": 3,
            "total_testcases": 3,
            "code_answer": ["1", "2"],
            "expected_code_answer": ["1", "2"],
            "status_msg": "Accepted",
            "status_runtime": "0 ms",
            "status_memory": "2 MB",
            "run_success": true,
            "runtime_percentile": 100.0,
            "memory_percentile": 99.5,
            "lang": "rust",
            "code_output": null,
            "full_runtime_error": null
        }"#;
        let r: TestSubmissionCheckResult = serde_json::from_str(raw).unwrap();
        assert_eq!(r.state, "SUCCESS");
        assert_eq!(r.correct_answer, Some(true));
        assert_eq!(r.total_correct, Some(3));
        assert_eq!(r.total_testcases, Some(3));
        assert_eq!(r.status_msg.as_deref(), Some("Accepted"));
        assert_eq!(r.lang.as_deref(), Some("rust"));
    }

    #[test]
    fn submission_check_result_wrong_answer_fields() {
        let raw = r#"{
            "state": "SUCCESS",
            "status_msg": "Wrong Answer",
            "code_output": "[3,0]",
            "expected_output": "[0,1]",
            "input": "[2,7,11,15]\n9",
            "total_correct": 0,
            "total_testcases": 57,
            "compile_error": null,
            "full_runtime_error": null
        }"#;
        let r: SubmissionCheckResult = serde_json::from_str(raw).unwrap();
        assert_eq!(r.status_msg.as_deref(), Some("Wrong Answer"));
        assert_eq!(r.code_output.as_deref(), Some("[3,0]"));
        assert_eq!(r.expected_output.as_deref(), Some("[0,1]"));
        assert_eq!(r.total_correct, Some(0));
        assert_eq!(r.total_testcases, Some(57));
    }

    #[test]
    fn question_deserializes_both_internal_and_frontend_ids() {
        let raw = r#"{
            "questionId": "1677",
            "questionFrontendId": "1550",
            "title": "Three Consecutive Odds",
            "titleSlug": "three-consecutive-odds",
            "content": "<p>Content</p>",
            "exampleTestcases": "[1,2,3]",
            "codeSnippets": [
                { "langSlug": "rust", "code": "impl Solution {}" }
            ]
        }"#;
        let q: Question = serde_json::from_str(raw).expect("question deserialization failed");
        assert_eq!(q.question_id, "1677");
        assert_eq!(q.question_frontend_id, "1550");
        assert_eq!(q.title_slug, "three-consecutive-odds");
        assert_eq!(q.title, "Three Consecutive Odds");
        assert_eq!(q.code_snippets.len(), 1);
    }
}
