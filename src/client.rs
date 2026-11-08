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
    fn is_terminal_failure(&self) -> Option<String> {
        None
    }
}

impl Pollable for crate::models::TestSubmissionCheckResult {
    fn is_complete(&self) -> bool {
        // LeetCode's state moves from "PENDING" or "STARTED" to "SUCCESS" when done
        self.state == "SUCCESS"
    }

    fn is_terminal_failure(&self) -> Option<String> {
        if self.state == "FAILURE" || self.state == "ERROR" {
            Some(format!("Test judging failed with state: {}", self.state))
        } else {
            None
        }
    }
}

impl Pollable for crate::models::SubmissionCheckResult {
    fn is_complete(&self) -> bool {
        self.state == "SUCCESS"
    }

    fn is_terminal_failure(&self) -> Option<String> {
        if self.state == "FAILURE" || self.state == "ERROR" {
            Some(format!(
                "Submission judging failed with state: {}",
                self.state
            ))
        } else {
            None
        }
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

    fn get_daily_challenge(
        &self,
    ) -> impl std::future::Future<Output = Result<crate::models::ActiveDailyQuestion>> + Send;
}

#[derive(serde::Deserialize, Debug)]
struct ProblemListResponse {
    stat_status_pairs: Option<Vec<StatStatusPair>>,
}

#[derive(serde::Deserialize, Debug)]
struct StatStatusPair {
    stat: Option<StatItem>,
    difficulty: Option<DifficultyItem>,
    status: Option<String>,
    paid_only: Option<bool>,
}

#[derive(serde::Deserialize, Debug)]
struct StatItem {
    question_id: Option<u64>,
    frontend_question_id: Option<u64>,
    #[serde(rename = "question__title")]
    question_title: Option<String>,
    #[serde(rename = "question__title_slug")]
    question_title_slug: Option<String>,
    total_acs: Option<u64>,
    total_submitted: Option<u64>,
}

#[derive(serde::Deserialize, Debug)]
struct DifficultyItem {
    level: Option<u64>,
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
            .connect_timeout(std::time::Duration::from_secs(10))
            .timeout(std::time::Duration::from_secs(30))
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

        let list_response: ProblemListResponse = response.json().await?;
        let mut target_slug = None;

        if let Some(pairs) = list_response.stat_status_pairs {
            for pair in pairs {
                if let Some(stat) = pair.stat
                    && stat.frontend_question_id == Some(id)
                {
                    target_slug = stat.question_title_slug;
                    break;
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
    /// deserialises to a type that reports [`Pollable::is_complete`] as `true`,
    /// or until the 40-attempt timeout (~60s) is reached.
    async fn poll_check<T: serde::de::DeserializeOwned + Pollable>(&self, url: &str) -> Result<T> {
        self.poll_check_internal(url, 40, 1500).await
    }

    /// Internal polling loop with configurable attempt count and sleep interval.
    async fn poll_check_internal<T: serde::de::DeserializeOwned + Pollable>(
        &self,
        url: &str,
        max_attempts: usize,
        interval_ms: u64,
    ) -> Result<T> {
        for attempt in 1..=max_attempts {
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

            if let Some(err_msg) = result.is_terminal_failure() {
                return Err(EngineError::Other(err_msg));
            }

            if attempt < max_attempts {
                tokio::time::sleep(std::time::Duration::from_millis(interval_ms)).await;
            }
        }

        Err(EngineError::Other(format!(
            "Timed out after {} attempts waiting for judge results",
            max_attempts
        )))
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

    /// Fetches today's Problem of the Day via the `questionOfToday` query.
    pub async fn get_daily_challenge(&self) -> Result<crate::models::ActiveDailyQuestion> {
        let query_string = r#"
                query questionOfToday {
                  activeDailyCodingChallengeQuestion {
                    date
                    question {
                      questionFrontendId
                      title
                      titleSlug
                      difficulty
                      isPaidOnly
                    }
                  }
                }
        "#;

        let query = GraphQLQuery {
            query: query_string.to_string(),
            variables: None,
            operation_name: Some("questionOfToday".to_string()),
        };

        let response: crate::models::DailyChallengeResponse = self.execute_graphql(query).await?;
        Ok(response.active_daily_coding_challenge_question)
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

        log::debug!("Fetching topic tags via GraphQL questionTopicTags...");
        let response: QuestionTopicWrapper = self.execute_graphql(query).await?;
        let response: Vec<QuestionTopics> = response
            .question_topic_tags
            .edges
            .iter()
            .map(|edge| edge.node.clone())
            .collect();
        log::debug!("Successfully loaded {} topic tags", response.len());
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
        log::debug!("Fetching problem list from {}...", url);

        let response = self.http_client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(crate::error::EngineError::GraphQL(format!(
                "Failed to fetch problem list: {}",
                response.status()
            )));
        }

        let list_response: ProblemListResponse = response.json().await?;

        if let Some(pairs) = list_response.stat_status_pairs {
            let mut problems = Vec::with_capacity(pairs.len());
            let mut id_map = std::collections::HashMap::with_capacity(pairs.len());

            for pair in pairs {
                if let (Some(stat), Some(difficulty)) = (pair.stat, pair.difficulty) {
                    let internal_id = stat.question_id.unwrap_or(0);
                    let frontend_id = stat.frontend_question_id.unwrap_or(0);

                    if internal_id > 0 && frontend_id > 0 {
                        id_map.insert(internal_id, frontend_id);

                        let title = stat.question_title.unwrap_or_default();
                        let slug = stat.question_title_slug.unwrap_or_default();
                        let level = difficulty.level.unwrap_or(0) as u8;
                        let status = pair.status;
                        let accepted = stat.total_acs.unwrap_or(0);
                        let submitted = stat.total_submitted.unwrap_or(0);
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
                            is_paid: pair.paid_only.unwrap_or(false),
                            topics: Vec::new(),
                        });
                    }
                }
            }
            // The API returns them sorted by ID descending by default, let's sort ascending
            problems.sort_by_key(|p| p.id);
            log::debug!(
                "Successfully loaded {} problems from LeetCode",
                problems.len()
            );
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

    fn get_daily_challenge(
        &self,
    ) -> impl std::future::Future<Output = Result<crate::models::ActiveDailyQuestion>> + Send {
        LeetCodeClient::get_daily_challenge(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    #[tokio::test]
    async fn test_poll_check_times_out_when_pending() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            loop {
                if let Ok((mut socket, _)) = listener.accept().await {
                    let mut buf = [0u8; 1024];
                    let _ = socket.read(&mut buf).await;
                    let body = r#"{"state":"PENDING"}"#;
                    let resp = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                        body.len(),
                        body
                    );
                    let _ = socket.write_all(resp.as_bytes()).await;
                }
            }
        });

        let creds = LeetCodeCredentials {
            session_cookie: "test".to_string(),
            csrf_token: "test".to_string(),
        };
        let client = LeetCodeClient::new(creds).unwrap();
        let url = format!("http://{}/check", addr);

        let res: Result<crate::models::SubmissionCheckResult> =
            client.poll_check_internal(&url, 3, 10).await;

        assert!(res.is_err());
        let err = res.unwrap_err().to_string();
        assert!(err.contains("Timed out after 3 attempts"));
    }

    #[tokio::test]
    async fn test_poll_check_fails_fast_on_terminal_failure() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            if let Ok((mut socket, _)) = listener.accept().await {
                let mut buf = [0u8; 1024];
                let _ = socket.read(&mut buf).await;
                let body = r#"{"state":"FAILURE"}"#;
                let resp = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                );
                let _ = socket.write_all(resp.as_bytes()).await;
            }
        });

        let creds = LeetCodeCredentials {
            session_cookie: "test".to_string(),
            csrf_token: "test".to_string(),
        };
        let client = LeetCodeClient::new(creds).unwrap();
        let url = format!("http://{}/check", addr);

        let res: Result<crate::models::SubmissionCheckResult> =
            client.poll_check_internal(&url, 5, 10).await;

        assert!(res.is_err());
        let err = res.unwrap_err().to_string();
        assert!(err.contains("Submission judging failed with state: FAILURE"));
    }

    #[tokio::test]
    async fn test_poll_check_succeeds_when_eventually_success() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            let mut count = 0;
            while let Ok((mut socket, _)) = listener.accept().await {
                count += 1;
                let mut buf = [0u8; 1024];
                let _ = socket.read(&mut buf).await;
                let body = if count == 1 {
                    r#"{"state":"STARTED"}"#
                } else {
                    r#"{"state":"SUCCESS","status_msg":"Accepted"}"#
                };
                let resp = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                );
                let _ = socket.write_all(resp.as_bytes()).await;
                if count >= 2 {
                    break;
                }
            }
        });

        let creds = LeetCodeCredentials {
            session_cookie: "test".to_string(),
            csrf_token: "test".to_string(),
        };
        let client = LeetCodeClient::new(creds).unwrap();
        let url = format!("http://{}/check", addr);

        let res: Result<crate::models::SubmissionCheckResult> =
            client.poll_check_internal(&url, 5, 10).await;

        assert!(res.is_ok());
        let result = res.unwrap();
        assert_eq!(result.state, "SUCCESS");
        assert_eq!(result.status_msg.as_deref(), Some("Accepted"));
    }

    #[test]
    fn test_problem_list_response_deserialization() {
        let json_data = r#"{
            "user_name": "test_user",
            "num_solved": 5,
            "stat_status_pairs": [
                {
                    "stat": {
                        "question_id": 100,
                        "frontend_question_id": 1,
                        "question__title": "Two Sum",
                        "question__title_slug": "two-sum",
                        "total_acs": 500,
                        "total_submitted": 1000,
                        "question__hide": false
                    },
                    "status": "ac",
                    "difficulty": { "level": 1 },
                    "paid_only": false,
                    "frequency": 0
                },
                {
                    "stat": {
                        "question_id": 101,
                        "frontend_question_id": 2,
                        "question__title": "Add Two Numbers",
                        "question__title_slug": "add-two-numbers",
                        "total_acs": 200,
                        "total_submitted": 500
                    },
                    "status": null,
                    "difficulty": { "level": 2 },
                    "paid_only": true
                }
            ]
        }"#;

        let parsed: ProblemListResponse = serde_json::from_str(json_data).unwrap();
        let pairs = parsed.stat_status_pairs.expect("stat_status_pairs present");
        assert_eq!(pairs.len(), 2);

        let p1 = &pairs[0];
        let s1 = p1.stat.as_ref().unwrap();
        assert_eq!(s1.question_id, Some(100));
        assert_eq!(s1.frontend_question_id, Some(1));
        assert_eq!(s1.question_title.as_deref(), Some("Two Sum"));
        assert_eq!(s1.question_title_slug.as_deref(), Some("two-sum"));
        assert_eq!(s1.total_acs, Some(500));
        assert_eq!(s1.total_submitted, Some(1000));
        assert_eq!(p1.status.as_deref(), Some("ac"));
        assert_eq!(p1.difficulty.as_ref().unwrap().level, Some(1));
        assert_eq!(p1.paid_only, Some(false));

        let p2 = &pairs[1];
        let s2 = p2.stat.as_ref().unwrap();
        assert_eq!(s2.question_id, Some(101));
        assert_eq!(s2.frontend_question_id, Some(2));
        assert_eq!(p2.status, None);
        assert_eq!(p2.difficulty.as_ref().unwrap().level, Some(2));
        assert_eq!(p2.paid_only, Some(true));
    }
}
