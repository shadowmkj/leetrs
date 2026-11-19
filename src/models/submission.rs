//! Submission payload and poll-result models.
use serde::{Deserialize, Serialize};

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

#[derive(Deserialize, Debug, Default)]
pub struct TestSubmissionCheckResult {
    pub code_answer: Option<Vec<String>>,
    pub code_output: Option<Vec<String>>,
    pub compile_error: Option<String>,
    pub correct_answer: Option<bool>,
    pub expected_code_answer: Option<Vec<String>>,
    pub full_compile_error: Option<String>,
    pub full_runtime_error: Option<String>,
    pub lang: Option<String>,
    pub memory_percentile: Option<f64>,
    pub run_success: Option<bool>,
    pub runtime_percentile: Option<f64>,
    pub state: String,
    pub status_memory: Option<String>,
    pub status_msg: Option<String>,
    pub status_runtime: Option<String>,
    pub std_output: Option<String>,
    pub std_output_list: Option<Vec<String>>,
    pub total_correct: Option<u32>,
    pub total_testcases: Option<u32>,
}

#[derive(Deserialize, Debug, Default)]
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
    pub std_output: Option<String>,
    pub std_output_list: Option<Vec<String>>,
    pub total_correct: Option<u32>,
    pub total_testcases: Option<u32>,
}
