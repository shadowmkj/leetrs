//! Authenticated HTTP client for the LeetCode API.
//!
//! All requests (REST and GraphQL) are made through [`LeetCodeClient`], which
//! pre-loads the required auth headers once at construction time so individual
//! methods don't have to deal with credential handling.
use crate::auth::LeetCodeCredentials;
use crate::error::{EngineError, Result};
use crate::models::{
    GraphQLQuery, Question, QuestionTopics, SubmissionCheckResult, SubmitPayload, SubmitResponse,
    TestPayload, TestSubmissionCheckResult, TestSubmitResponse, UserDetail,
};
use reqwest::Client;
use reqwest::header::{COOKIE, HeaderMap, HeaderValue, USER_AGENT};
use serde_json::json;

/// Implemented by any poll-result type that has a terminal "complete" state.
///
/// Used by the generic [`LeetCodeClient::poll_check`] method to avoid
/// duplicating the polling loop for test submissions and full submissions.
trait Pollable {
    fn is_complete(&self) -> bool;
}

impl Pollable for crate::models::TestSubmissionCheckResult {
    fn is_complete(&self) -> bool {
        // LeetCode's state moves from "PENDING" or "STARTED" to "SUCCESS" when done
        self.state == "SUCCESS"
    }
}

impl Pollable for crate::models::SubmissionCheckResult {
    fn is_complete(&self) -> bool {
        self.state == "SUCCESS"
    }
}

/// Abstraction over the LeetCode HTTP/GraphQL API.
///
/// Implementing this trait for a mock type allows [`Picker`], [`SubmissionService`],
/// and the TUI to be tested without hitting the real network.
pub trait LeetCodeApi {
    fn get_question_by_slug(
        &self,
        title_slug: &str,
    ) -> impl std::future::Future<Output = Result<crate::models::Question>> + Send;

    fn get_question_by_id(
        &self,
        id: u64,
    ) -> impl std::future::Future<Output = Result<crate::models::Question>> + Send;

    fn test_code(
        &self,
        title_slug: &str,
        question_id: &str,
        lang: &str,
        code: &str,
        test_cases: &str,
    ) -> impl std::future::Future<Output = Result<String>> + Send;

    fn submit_code(
        &self,
        title_slug: &str,
        question_id: &str,
        lang: &str,
        code: &str,
    ) -> impl std::future::Future<Output = Result<u64>> + Send;

    fn check_test_submission(
        &self,
        interpret_id: String,
    ) -> impl std::future::Future<Output = Result<crate::models::TestSubmissionCheckResult>> + Send;

    fn check_submission(
        &self,
        submission_id: u64,
    ) -> impl std::future::Future<Output = Result<crate::models::SubmissionCheckResult>> + Send;

    fn get_user_detail(
        &self,
    ) -> impl std::future::Future<Output = Result<crate::models::UserDetail>> + Send;

    fn get_topics_question_list(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<crate::models::QuestionTopics>>> + Send;

    fn get_problem_list(
        &self,
    ) -> impl std::future::Future<
        Output = Result<(
            Vec<crate::models::ProblemSummary>,
            std::collections::HashMap<u64, u64>,
        )>,
    > + Send;
}

/// A configured `reqwest` HTTP client that authenticates every request with
/// the user's LeetCode session cookies and CSRF token.
///
/// Construct once with [`LeetCodeClient::new`] and share via clone or `Rc`.
#[derive(Clone, Debug)]
pub struct LeetCodeClient {
    http_client: Client,
}

impl LeetCodeClient {
    /// Builds a new client, baking the session cookie and CSRF token directly
    /// into the default headers of every subsequent request.
    pub fn new(creds: LeetCodeCredentials) -> Result<Self> {
        let mut headers = HeaderMap::new();

        // Construct the cookie string
        let cookie_str = format!(
            "LEETCODE_SESSION={}; csrftoken={}",
            creds.session_cookie, creds.csrf_token
        );

        headers.insert(COOKIE, HeaderValue::from_str(&cookie_str).unwrap());
        headers.insert(
            "x-csrftoken",
            HeaderValue::from_str(&creds.csrf_token).unwrap(),
        );
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static(
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36",
            ),
        );
        headers.insert("Referer", HeaderValue::from_static("https://leetcode.com/"));

        let client = Client::builder()
            .default_headers(headers)
            .cookie_store(true)
            .build()?;

        Ok(Self {
            http_client: client,
        })
    }

    /// Sends a GraphQL query to LeetCode and deserialises the `data` field of
    /// the response into `T`.
    ///
    /// Consumes the `GraphQLQuery` struct and handles both HTTP-level errors
    /// and LeetCode-level GraphQL errors (`{ "errors": [...] }`).
    pub async fn execute_graphql<T: serde::de::DeserializeOwned>(
        &self,
        query: GraphQLQuery,
    ) -> Result<T> {
        let response = self
            .http_client
            .post("https://leetcode.com/graphql")
            .json(&query)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(EngineError::Network(
                response.error_for_status().unwrap_err(),
            ));
        }

        let mut json_data: serde_json::Value = response.json().await?;

        if let Some(errors) = json_data.get("errors") {
            return Err(EngineError::GraphQL(errors.to_string()));
        }

        let data = json_data
            .get_mut("data")
            .map(std::mem::take)
            .ok_or_else(|| EngineError::GraphQL("Missing data field".into()))?;

        serde_json::from_value(data).map_err(EngineError::from)
    }

    /// Fetch a specific problem's details and boilerplate using its URL slug
    pub async fn get_question_by_slug(&self, title_slug: &str) -> Result<Question> {
        // 1. Define the exact GraphQL query LeetCode expects.
        // We request both `questionId` (the internal database ID required by LeetCode's
        // submission judge endpoints) and `questionFrontendId` (the public problem number
        // shown to users in the UI and written in problem metadata headers).
        let query_string = r#"
            query questionData($titleSlug: String!) {
                question(titleSlug: $titleSlug) {
                    questionId
                    questionFrontendId
                    title
                    titleSlug
                    content
                    exampleTestcases
                    codeSnippets {
                        langSlug
                        code
                    }
                }
            }
        "#;

        // 2. Construct the payload with the dynamic slug variable
        let query = GraphQLQuery {
            query: query_string.to_string(),
            variables: Some(json!({ "titleSlug": title_slug })),
            operation_name: Some("questionData".to_string()),
        };

        // 3. Create a temporary wrapper to handle the nested JSON response
        // LeetCode returns: { "data": { "question": { ... } } }
        // Our execute_graphql method strips the "data" layer, so we catch the "question" layer here.
        #[derive(serde::Deserialize)]
        struct QuestionWrapper {
            question: Question,
        }

        // 4. Execute the query and unwrap the final Question struct
        let response: QuestionWrapper = self.execute_graphql(query).await?;

        Ok(response.question)
    }
    /// Fetch a specific problem's details and boilerplate using its numerical ID
    pub async fn get_question_by_id(&self, id: u64) -> Result<Question> {
        let url = "https://leetcode.com/api/problems/all/";

        let response = self.http_client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(EngineError::GraphQL(format!(
                "Failed to fetch problem list: {}",
                response.status()
            )));
        }

        let json_data: serde_json::Value = response.json().await?;
        let mut target_slug = None;

        // Traverse the generic JSON tree to find the matching ID
        if let Some(pairs) = json_data
            .get("stat_status_pairs")
            .and_then(|v| v.as_array())
        {
            for pair in pairs {
                if let Some(stat) = pair.get("stat") {
                    let current_id = stat.get("frontend_question_id").and_then(|v| v.as_u64());

                    if current_id == Some(id)
                        && let Some(slug) =
                            stat.get("question__title_slug").and_then(|v| v.as_str())
                    {
                        target_slug = Some(slug.to_string());
                        break;
                    }
                }
            }
        }

        let slug = target_slug
            .ok_or_else(|| EngineError::GraphQL(format!("Problem with ID {} not found", id)))?;

        // Chain directly into the slug fetcher, returning Result<Question>
        self.get_question_by_slug(&slug).await
    }

    /// Test raw code to a problem
    pub async fn test_code(
        &self,
        title_slug: &str,
        question_id: &str,
        lang: &str,
        code: &str,
        test_cases: &str,
    ) -> Result<String> {
        let url = format!(
            "https://leetcode.com/problems/{}/interpret_solution/",
            title_slug
        );

        let payload = TestPayload {
            lang: lang.to_string(),
            question_id: question_id.to_string(),
            typed_code: code.to_string(),
            data_input: test_cases.to_string(),
        };

        let response = self
            .http_client
            .post(&url)
            .json(&payload)
            // Critical: LeetCode requires the Referer header to match the problem page to bypass CSRF checks
            .header(
                "Referer",
                format!("https://leetcode.com/problems/{}/", title_slug),
            )
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(EngineError::GraphQL(format!(
                "Test Submission failed: {}",
                response.status()
            )));
        }

        let result: TestSubmitResponse = response.json().await?;
        Ok(result.interpret_id)
    }

    /// Submit raw code to a problem
    pub async fn submit_code(
        &self,
        title_slug: &str,
        question_id: &str,
        lang: &str,
        code: &str,
    ) -> Result<u64> {
        let url = format!("https://leetcode.com/problems/{}/submit/", title_slug);

        let payload = SubmitPayload {
            lang: lang.to_string(),
            question_id: question_id.to_string(),
            typed_code: code.to_string(),
        };

        let response = self
            .http_client
            .post(&url)
            .json(&payload)
            // Critical: LeetCode requires the Referer header to match the problem page to bypass CSRF checks
            .header(
                "Referer",
                format!("https://leetcode.com/problems/{}/", title_slug),
            )
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(EngineError::GraphQL(format!(
                "Submission failed: {}",
                response.status()
            )));
        }

        let result: SubmitResponse = response.json().await?;
        Ok(result.submission_id)
    }

    /// Poll the test submission status until it completes
    pub async fn check_test_submission(
        &self,
        interpret_id: String,
    ) -> Result<TestSubmissionCheckResult> {
        let url = format!(
            "https://leetcode.com/submissions/detail/{}/check/",
            interpret_id
        );
        self.poll_check(&url).await
    }

    /// Poll the submission status until it completes
    pub async fn check_submission(&self, submission_id: u64) -> Result<SubmissionCheckResult> {
        let url = format!(
            "https://leetcode.com/submissions/detail/{}/check/",
            submission_id
        );
        self.poll_check(&url).await
    }

    /// Generic polling loop shared by [`check_test_submission`] and [`check_submission`].
    ///
    /// Keeps sending GET requests to `url` every 1.5 s until the response
    /// deserialises to a type that reports [`Pollable::is_complete`] as `true`.
    async fn poll_check<T: serde::de::DeserializeOwned + Pollable>(&self, url: &str) -> Result<T> {
        loop {
            let response = self.http_client.get(url).send().await?;

            if !response.status().is_success() {
                return Err(EngineError::GraphQL(format!(
                    "Check failed: {}",
                    response.status()
                )));
            }

            let result: T = response.json().await?;

            if result.is_complete() {
                return Ok(result);
            }

            // Sleep for 1.5 seconds before polling again to avoid hitting rate limits
            tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
        }
    }

    /// Fetches the current user's profile via the `userStatus` GraphQL query.
    pub async fn get_user_detail(&self) -> Result<UserDetail> {
        let query_string = r#"
                query {
                  userStatus {
                    username
                    isPremium
                    isVerified
                  }
                }
        "#;

        let query = GraphQLQuery {
            query: query_string.to_string(),
            variables: None,
            operation_name: None,
        };

        #[derive(serde::Deserialize)]
        struct UserDetailWrapper {
            #[serde(rename = "userStatus")]
            user_status: UserDetail,
        }

        let response: UserDetailWrapper = self.execute_graphql(query).await?;
        Ok(response.user_status)
    }

    /// Fetches all topic tags along with the set of problem IDs tagged under each.
    ///
    /// The response is deeply nested (`questionTopicTags > edges > node`); local
    /// wrapper structs are used to drive serde without polluting the public API.
    pub async fn get_topics_question_list(&self) -> Result<Vec<crate::models::QuestionTopics>> {
        let query_string = r#"
        query questionTopicTags {
  questionTopicTags {
    edges {
      node {
        id
        name
        slug
        translatedName
        questionIds
      }
    }
  }
}
        "#;

        let query = GraphQLQuery {
            query: query_string.to_string(),
            variables: None,
            operation_name: Some("questionTopicTags".to_string()),
        };

        // Create a temporary wrapper to handle the nested JSON response
        // LeetCode returns: { "data": { "question": { ... } } }
        // Our execute_graphql method strips the "data" layer, so we catch the "question" layer here.
        #[derive(serde::Deserialize)]
        struct Something {
            node: QuestionTopics,
        }
        #[derive(serde::Deserialize)]
        struct Edge {
            edges: Vec<Something>,
        }
        #[derive(serde::Deserialize)]
        struct QuestionTopicWrapper {
            #[serde(rename = "questionTopicTags")]
            question_topic_tags: Edge,
        }

        let response: QuestionTopicWrapper = self.execute_graphql(query).await?;
        let response: Vec<QuestionTopics> = response
            .question_topic_tags
            .edges
            .iter()
            .map(|edge| edge.node.clone())
            .collect();
        Ok(response)
    }

    /// Fetches the master list of all LeetCode problems along with a mapping
    /// from internal `question_id` to public `frontend_question_id`.
    pub async fn get_problem_list(
        &self,
    ) -> Result<(
        Vec<crate::models::ProblemSummary>,
        std::collections::HashMap<u64, u64>,
    )> {
        let url = "https://leetcode.com/api/problems/all/";

        let response = self.http_client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(crate::error::EngineError::GraphQL(format!(
                "Failed to fetch problem list: {}",
                response.status()
            )));
        }

        let json_data: serde_json::Value = response.json().await?;

        if let Some(pairs) = json_data
            .get("stat_status_pairs")
            .and_then(|v| v.as_array())
        {
            let mut problems = Vec::with_capacity(pairs.len());
            let mut id_map = std::collections::HashMap::with_capacity(pairs.len());

            for pair in pairs {
                if let (Some(stat), Some(difficulty), Some(status), Some(paid_only)) = (
                    pair.get("stat"),
                    pair.get("difficulty"),
                    pair.get("status"),
                    pair.get("paid_only"),
                ) {
                    let internal_id = stat
                        .get("question_id")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(0);
                    let frontend_id = stat
                        .get("frontend_question_id")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(0);

                    if internal_id > 0 && frontend_id > 0 {
                        id_map.insert(internal_id, frontend_id);

                        let title = stat
                            .get("question__title")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();
                        let slug = stat
                            .get("question__title_slug")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();
                        let level = difficulty
                            .get("level")
                            .and_then(|v| v.as_u64())
                            .unwrap_or(0) as u8;
                        let status = status.as_str().map(String::from);
                        let accepted = stat.get("total_acs").and_then(|v| v.as_u64()).unwrap_or(0);
                        let submitted = stat
                            .get("total_submitted")
                            .and_then(|v| v.as_u64())
                            .unwrap_or(0);
                        // Guard against division by zero for problems with no submissions yet.
                        let acceptance = if submitted > 0 {
                            accepted as f64 / submitted as f64
                        } else {
                            0.0
                        };

                        problems.push(crate::models::ProblemSummary {
                            id: frontend_id,
                            title,
                            slug,
                            difficulty: level,
                            accepted,
                            submitted,
                            acceptance,
                            status,
                            is_paid: paid_only.as_bool().unwrap_or(false),
                            topics: Vec::new(),
                        });
                    }
                }
            }
            // The API returns them sorted by ID descending by default, let's sort ascending
            problems.sort_by_key(|p| p.id);
            return Ok((problems, id_map));
        }
        Err(EngineError::Other(
            "Error retrieving problem list".to_string(),
        ))
    }
}

/// Blanket [`LeetCodeApi`] implementation for the concrete HTTP client.
///
/// Each method simply forwards to the same method on [`LeetCodeClient`], so
/// there is no logic duplication — the trait is purely an abstraction seam
/// that lets tests inject a mock without touching `reqwest`.
impl LeetCodeApi for LeetCodeClient {
    fn get_question_by_slug(
        &self,
        title_slug: &str,
    ) -> impl std::future::Future<Output = Result<crate::models::Question>> + Send {
        LeetCodeClient::get_question_by_slug(self, title_slug)
    }

    fn get_question_by_id(
        &self,
        id: u64,
    ) -> impl std::future::Future<Output = Result<crate::models::Question>> + Send {
        LeetCodeClient::get_question_by_id(self, id)
    }

    fn test_code(
        &self,
        title_slug: &str,
        question_id: &str,
        lang: &str,
        code: &str,
        test_cases: &str,
    ) -> impl std::future::Future<Output = Result<String>> + Send {
        LeetCodeClient::test_code(self, title_slug, question_id, lang, code, test_cases)
    }

    fn submit_code(
        &self,
        title_slug: &str,
        question_id: &str,
        lang: &str,
        code: &str,
    ) -> impl std::future::Future<Output = Result<u64>> + Send {
        LeetCodeClient::submit_code(self, title_slug, question_id, lang, code)
    }

    fn check_test_submission(
        &self,
        interpret_id: String,
    ) -> impl std::future::Future<Output = Result<crate::models::TestSubmissionCheckResult>> + Send
    {
        LeetCodeClient::check_test_submission(self, interpret_id)
    }

    fn check_submission(
        &self,
        submission_id: u64,
    ) -> impl std::future::Future<Output = Result<crate::models::SubmissionCheckResult>> + Send
    {
        LeetCodeClient::check_submission(self, submission_id)
    }

    fn get_user_detail(
        &self,
    ) -> impl std::future::Future<Output = Result<crate::models::UserDetail>> + Send {
        LeetCodeClient::get_user_detail(self)
    }

    fn get_topics_question_list(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<crate::models::QuestionTopics>>> + Send {
        LeetCodeClient::get_topics_question_list(self)
    }

    fn get_problem_list(
        &self,
    ) -> impl std::future::Future<
        Output = Result<(
            Vec<crate::models::ProblemSummary>,
            std::collections::HashMap<u64, u64>,
        )>,
    > + Send {
        LeetCodeClient::get_problem_list(self)
    }
}
