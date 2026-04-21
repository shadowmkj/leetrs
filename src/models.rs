//! Data models shared across the crate.
//!
//! All types here are either serialised into HTTP request bodies or deserialised
//! from API responses. Serde field renames mirror LeetCode's camelCase JSON keys.
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

/// A GraphQL request body sent to `https://leetcode.com/graphql`.
#[derive(Serialize, Debug)]
pub struct GraphQLQuery {
    pub query: String,
    pub variables: Option<serde_json::Value>,
    #[serde(rename = "operationName")]
    pub operation_name: Option<String>,
}

/// Supported submission languages.
///
/// The `ValueEnum` derive lets Clap accept these directly as CLI arguments.
#[derive(Debug, Clone, ValueEnum)]
pub enum Language {
    Python,
    Rust,
    Pandas,
    Mysql,
    Postgres,
}

/// A problem identifier supplied on the command line — either a numeric ID or a slug.
#[derive(Debug, Clone)]
pub enum Identifier {
    Number(u64),
    String(String),
}

impl From<String> for Language {
    fn from(value: String) -> Self {
        match value.as_str() {
            "python3" => Self::Python,
            "rust" => Self::Rust,
            "pythondata" => Self::Pandas,
            "mysql" => Self::Mysql,
            "postgresql" => Self::Postgres,
            _ => Self::Mysql,
        }
    }
}

impl Language {
    /// Maps a [`Language`] variant to LeetCode's internal language slug string.
    pub fn to_lang_slug(&self) -> &'static str {
        match self {
            Language::Python => "python3",
            Language::Rust => "rust",
            Language::Mysql => "mysql",
            Language::Pandas => "pythondata",
            Language::Postgres => "postgresql",
        }
    }

    /// Infers the language from a file extension. Falls back to MySQL for unknown extensions.
    pub fn from_extension(ext: &str) -> Self {
        match ext {
            "py" => Language::Python,
            "rs" => Language::Rust,
            "sql" => Language::Mysql,
            _ => Language::Mysql,
        }
    }
}

/// A single code snippet returned by LeetCode's GraphQL API for one language.
#[derive(Deserialize, Debug)]
pub struct QuestionSnippet {
    #[serde(rename = "langSlug")]
    pub lang_slug: String,
    pub code: String,
}

/// Minimal user profile returned by the `userStatus` GraphQL query.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserDetail {
    pub username: Option<String>,
    #[serde(rename = "isPremium")]
    pub is_premium: Option<bool>,
    #[serde(rename = "isVerified")]
    pub is_verified: bool,
}

/// Full problem details fetched from the `questionData` GraphQL query.
#[derive(Deserialize, Debug)]
pub struct Question {
    #[serde(rename = "questionId")]
    pub question_id: String,
    #[serde(rename = "titleSlug")]
    pub title_slug: String,
    pub title: String,
    pub content: String,
    #[serde(rename = "exampleTestcases")]
    pub example_test_cases: String,
    #[serde(rename = "codeSnippets")]
    pub code_snippets: Vec<QuestionSnippet>,
}

/// Lightweight problem summary used to populate the TUI problem list.
///
/// These are deserialized from the cached `data.json` file and from the
/// `/api/problems/all/` REST endpoint.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProblemSummary {
    pub id: u64,
    pub acceptance: f64,
    pub accepted: u64,
    /// Difficulty level: `1` = Easy, `2` = Medium, `3` = Hard.
    pub difficulty: u8,
    pub slug: String,
    /// `"ac"` if solved, `"notac"` if attempted but not solved, `None` if untouched.
    pub status: Option<String>,
    pub submitted: u64,
    pub title: String,
    pub is_paid: bool,
    pub topics: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Topic {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QuestionTopics {
    pub name: String,
    pub id: String,
    pub slug: String,
    #[serde(rename = "translatedName")]
    pub translated_name: Option<String>,
    #[serde(rename = "questionIds")]
    pub question_ids: Vec<u64>,
}

// ==========================================
// Submission Models
// ==========================================

#[derive(Serialize, Debug)]
pub struct SubmitPayload {
    pub lang: String,
    pub question_id: String,
    pub typed_code: String,
}

/// Request body sent to `/problems/{slug}/interpret_solution/` (test run).
#[derive(Serialize, Debug)]
pub struct TestPayload {
    pub lang: String,
    pub question_id: String,
    pub typed_code: String,
    /// The raw test-case input string used by LeetCode's judge.
    pub data_input: String,
}

/// Response from `/interpret_solution/` — contains the ID used to poll for results.
#[derive(Deserialize, Debug)]
pub struct TestSubmitResponse {
    /// Opaque ID used to poll `/submissions/detail/{interpret_id}/check/`.
    pub interpret_id: String,
    pub test_case: String,
}

/// Response from `/problems/{slug}/submit/` — contains the submission ID for polling.
#[derive(Deserialize, Debug)]
pub struct SubmitResponse {
    pub submission_id: u64,
}

#[derive(Deserialize, Debug)]
pub struct TestSubmissionCheckResult {
    pub code_answer: Option<Vec<String>>,
    pub code_output: Option<Vec<String>>,
    pub correct_answer: Option<bool>,
    pub expected_code_answer: Option<Vec<String>>,
    pub full_runtime_error: Option<String>,
    pub lang: Option<String>,
    pub memory_percentile: Option<f64>,
    pub run_success: Option<bool>,
    pub runtime_percentile: Option<f64>,
    pub state: String,
    pub status_memory: Option<String>,
    pub status_msg: Option<String>,
    pub status_runtime: Option<String>,
    pub total_correct: Option<u32>,
    pub total_testcases: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct SubmissionCheckResult {
    pub code_output: Option<String>,
    pub compile_error: Option<String>,
    pub expected_output: Option<String>,
    pub finished: Option<bool>,
    pub full_runtime_error: Option<String>,
    pub input: Option<String>,
    pub input_formatted: Option<String>,
    pub last_testcase: Option<String>,
    pub memory_percentile: Option<f64>,
    pub run_success: Option<bool>,
    pub runtime_percentile: Option<f64>,
    pub state: String, // "PENDING", "STARTED", "SUCCESS"
    pub status_memory: Option<String>,
    pub status_msg: Option<String>, // "Accepted", "Wrong Answer", "Compile Error"
    pub status_runtime: Option<String>,
    pub total_correct: Option<u32>,
    pub total_testcases: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    // -----------------------------------------------------------------------
    // Language::to_lang_slug
    // -----------------------------------------------------------------------

    #[test]
    fn to_lang_slug_python() {
        assert_eq!(Language::Python.to_lang_slug(), "python3");
    }

    #[test]
    fn to_lang_slug_rust() {
        assert_eq!(Language::Rust.to_lang_slug(), "rust");
    }

    #[test]
    fn to_lang_slug_pandas() {
        assert_eq!(Language::Pandas.to_lang_slug(), "pythondata");
    }

    #[test]
    fn to_lang_slug_mysql() {
        assert_eq!(Language::Mysql.to_lang_slug(), "mysql");
    }

    #[test]
    fn to_lang_slug_postgres() {
        assert_eq!(Language::Postgres.to_lang_slug(), "postgresql");
    }

    // -----------------------------------------------------------------------
    // Language::from (via lang_slug string returned by LeetCode)
    // -----------------------------------------------------------------------

    #[test]
    fn language_from_python3_slug() {
        assert!(matches!(Language::from("python3".to_string()), Language::Python));
    }

    #[test]
    fn language_from_rust_slug() {
        assert!(matches!(Language::from("rust".to_string()), Language::Rust));
    }

    #[test]
    fn language_from_pythondata_slug() {
        assert!(matches!(Language::from("pythondata".to_string()), Language::Pandas));
    }

    #[test]
    fn language_from_mysql_slug() {
        assert!(matches!(Language::from("mysql".to_string()), Language::Mysql));
    }

    #[test]
    fn language_from_postgresql_slug() {
        assert!(matches!(Language::from("postgresql".to_string()), Language::Postgres));
    }

    /// Unknown slugs must fall back to Mysql rather than panicking.
    #[test]
    fn language_from_unknown_slug_falls_back_to_mysql() {
        assert!(matches!(Language::from("javascript".to_string()), Language::Mysql));
        assert!(matches!(Language::from("".to_string()), Language::Mysql));
        assert!(matches!(Language::from("PYTHON3".to_string()), Language::Mysql)); // case-sensitive
    }

    // -----------------------------------------------------------------------
    // Language::from_extension
    // -----------------------------------------------------------------------

    #[test]
    fn from_extension_py_is_python() {
        assert!(matches!(Language::from_extension("py"), Language::Python));
    }

    #[test]
    fn from_extension_rs_is_rust() {
        assert!(matches!(Language::from_extension("rs"), Language::Rust));
    }

    #[test]
    fn from_extension_sql_is_mysql() {
        assert!(matches!(Language::from_extension("sql"), Language::Mysql));
    }

    /// Unknown extensions must fall back gracefully, not panic.
    #[test]
    fn from_extension_unknown_falls_back_to_mysql() {
        assert!(matches!(Language::from_extension("js"), Language::Mysql));
        assert!(matches!(Language::from_extension(""), Language::Mysql));
        assert!(matches!(Language::from_extension("txt"), Language::Mysql));
    }

    // -----------------------------------------------------------------------
    // to_lang_slug ↔ from round-trip
    // -----------------------------------------------------------------------

    fn roundtrip(lang: Language) -> bool {
        // Converts a Language variant → LeetCode slug string → back to Language,
        // then checks the slug string is preserved.
        let slug = lang.to_lang_slug();
        let recovered = Language::from(slug.to_string());
        recovered.to_lang_slug() == slug
    }

    #[test]
    fn lang_slug_roundtrip_all_variants() {
        assert!(roundtrip(Language::Python));
        assert!(roundtrip(Language::Rust));
        assert!(roundtrip(Language::Pandas));
        assert!(roundtrip(Language::Mysql));
        assert!(roundtrip(Language::Postgres));
    }

    // -----------------------------------------------------------------------
    // Serde: ProblemSummary
    // -----------------------------------------------------------------------

    #[test]
    fn problem_summary_serde_roundtrip() {
        let ps = ProblemSummary {
            id: 1,
            acceptance: 0.55,
            accepted: 5500,
            difficulty: 2,
            slug: "two-sum".to_string(),
            status: Some("ac".to_string()),
            submitted: 10000,
            title: "Two Sum".to_string(),
            is_paid: false,
            topics: vec!["Array".to_string(), "Hash Table".to_string()],
        };
        let json = serde_json::to_string(&ps).expect("serialization failed");
        let back: ProblemSummary = serde_json::from_str(&json).expect("deserialization failed");
        assert_eq!(back.id, 1);
        assert_eq!(back.slug, "two-sum");
        assert!((back.acceptance - 0.55).abs() < 1e-9);
        assert_eq!(back.difficulty, 2);
        assert_eq!(back.status.as_deref(), Some("ac"));
        assert_eq!(back.topics, vec!["Array", "Hash Table"]);
        assert!(!back.is_paid);
    }

    #[test]
    fn problem_summary_no_status_roundtrip() {
        let ps = ProblemSummary {
            id: 42,
            acceptance: 0.0,
            accepted: 0,
            difficulty: 1,
            slug: "fizz-buzz".to_string(),
            status: None,
            submitted: 0,
            title: "Fizz Buzz".to_string(),
            is_paid: true,
            topics: vec![],
        };
        let json = serde_json::to_string(&ps).unwrap();
        let back: ProblemSummary = serde_json::from_str(&json).unwrap();
        assert!(back.status.is_none());
        assert!(back.topics.is_empty());
        assert!(back.is_paid);
    }

    // -----------------------------------------------------------------------
    // Serde: UserDetail (camelCase rename sanity)
    // -----------------------------------------------------------------------

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
    fn user_detail_serde_roundtrip() {
        let ud = UserDetail {
            username: Some("bob".to_string()),
            is_premium: Some(false),
            is_verified: true,
        };
        let json = serde_json::to_string(&ud).unwrap();
        let back: UserDetail = serde_json::from_str(&json).unwrap();
        assert_eq!(back.username.as_deref(), Some("bob"));
        assert_eq!(back.is_premium, Some(false));
        assert!(back.is_verified);
    }

    // -----------------------------------------------------------------------
    // Serde: TestSubmissionCheckResult / SubmissionCheckResult
    // -----------------------------------------------------------------------

    #[test]
    fn test_submission_check_result_minimal_deserialize() {
        // Only the required `state` field is set; everything optional should be None.
        let raw = r#"{"state":"PENDING"}"#;
        let r: TestSubmissionCheckResult =
            serde_json::from_str(raw).expect("should accept minimal structure");
        assert_eq!(r.state, "PENDING");
        assert!(r.correct_answer.is_none());
        assert!(r.total_correct.is_none());
        assert!(r.code_answer.is_none());
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
        assert!((r.runtime_percentile.unwrap() - 100.0).abs() < 1e-9);
        assert!((r.memory_percentile.unwrap() - 99.5).abs() < 1e-9);
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
        assert!(r.input.as_deref().unwrap().contains("[2,7,11,15]"));
        assert_eq!(r.total_correct, Some(0));
        assert_eq!(r.total_testcases, Some(57));
    }

    // -----------------------------------------------------------------------
    // GraphQLQuery serialization
    // -----------------------------------------------------------------------

    #[test]
    fn graphql_query_serializes_with_variables() {
        let q = GraphQLQuery {
            query: "query example { foo }".to_string(),
            variables: Some(serde_json::json!({ "id": 1 })),
            operation_name: Some("example".to_string()),
        };
        let json = serde_json::to_string(&q).unwrap();
        assert!(json.contains("operationName"));
        assert!(json.contains("example"));
        assert!(json.contains("variables"));
    }

    #[test]
    fn graphql_query_serializes_without_optional_fields() {
        let q = GraphQLQuery {
            query: "{ userStatus { username } }".to_string(),
            variables: None,
            operation_name: None,
        };
        let json = serde_json::to_string(&q).unwrap();
        // None fields should serialize as `null`, not be omitted (no #[serde(skip_serializing_if)])
        assert!(json.contains("variables"));
        assert!(json.contains("operationName"));
    }
}
