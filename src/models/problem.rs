//! Problem summary, question detail, and topic models.
use serde::{Deserialize, Serialize};

/// A GraphQL request body sent to `https://leetcode.com/graphql`.
#[derive(Serialize, Debug)]
pub struct GraphQLQuery {
    pub query: String,
    pub variables: Option<serde_json::Value>,
    #[serde(rename = "operationName")]
    pub operation_name: Option<String>,
}

/// A single code snippet returned by LeetCode's GraphQL API for one language.
#[derive(Serialize, Deserialize, Clone, Debug)]
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
///
/// # Identifier Distinction: `question_id` vs `question_frontend_id`
///
/// LeetCode maintains two distinct identifiers for each problem:
/// - [`question_id`](Self::question_id) (`questionId` in GraphQL / REST):
///   The internal database primary key. LeetCode's backend judging endpoints
///   (`/problems/{slug}/submit/` and `/interpret_solution/`) **require** this
///   internal ID in their JSON submission payloads.
/// - [`question_frontend_id`](Self::question_frontend_id) (`questionFrontendId` in GraphQL / `frontend_question_id` in REST):
///   The public-facing problem number visible on leetcode.com, in problem tables,
///   and in search results (e.g. `1` for Two Sum, `1550` for Three Consecutive Odds).
///   This ID is used for user display and in generated source file metadata headers.
#[derive(Deserialize, Debug, Clone)]
pub struct Question {
    /// Internal database ID required by LeetCode judging/submission APIs.
    #[serde(rename = "questionId")]
    pub question_id: String,
    /// Public problem number visible to users on LeetCode and in search.
    #[serde(rename = "questionFrontendId")]
    pub question_frontend_id: String,
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
