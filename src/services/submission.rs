//! Submission service — orchestrates the submit/test workflow.
//!
//! Extracts the shared logic between [`Picker::test_submit`] and
//! [`Picker::submit`] into a single `submit_or_test` method with a structured
//! result type that callers can pattern-match on.
use std::path::Path;

use crate::{
    client::LeetCodeClient,
    error::{EngineError, Result},
    models::{Language, Question},
};

// ============================================================================
// Result types
// ============================================================================

/// The outcome of a LeetCode submission or test run.
#[derive(Debug, Clone, PartialEq)]
pub enum SubmissionStatus {
    Accepted,
    WrongAnswer,
    CompileError,
    RuntimeError,
    TimeLimitExceeded,
    Unknown(String),
}

impl SubmissionStatus {
    /// Parses a LeetCode `status_msg` string into the enum variant.
    pub fn from_msg(msg: &str) -> Self {
        match msg {
            "Accepted" => Self::Accepted,
            "Wrong Answer" => Self::WrongAnswer,
            "Compile Error" => Self::CompileError,
            "Runtime Error" => Self::RuntimeError,
            "Time Limit Exceeded" => Self::TimeLimitExceeded,
            other => Self::Unknown(other.to_string()),
        }
    }
}

/// The full, structured result of a submission or test run.
///
/// Callers can inspect the `status` variant and the optional details fields
/// to render results however they like (CLI `println!`, TUI popup, etc.).
#[derive(Debug, Clone)]
pub struct SubmissionResult {
    pub status: SubmissionStatus,
    pub is_test: bool,
    // Test-run specific
    pub correct_answer: Option<bool>,
    pub code_answer: Option<Vec<String>>,
    pub expected_code_answer: Option<Vec<String>>,
    // Shared
    pub total_correct: Option<u32>,
    pub total_testcases: Option<u32>,
    pub runtime: Option<String>,
    pub memory: Option<String>,
    pub memory_percentile: Option<f64>,
    pub runtime_percentile: Option<f64>,
    pub compile_error: Option<String>,
    pub full_runtime_error: Option<String>,
    // Submit-run specific
    pub input: Option<String>,
    pub expected_output: Option<String>,
    pub code_output: Option<String>,
}

// ============================================================================
// Service
// ============================================================================

/// Encapsulates the file-read → slug-resolve → API-call flow for both
/// `submit` and `test` operations.
pub struct SubmissionService {
    client: LeetCodeClient,
}

impl SubmissionService {
    pub fn new(client: LeetCodeClient) -> Self {
        Self { client }
    }

    /// Reads the source file, derives the problem slug and language from the
    /// filename/extension, fetches the full question metadata, and returns the
    /// tuple `(code, slug, language, question)` ready for submission.
    async fn read_and_resolve(&self, file: &str) -> Result<(String, String, Language, Question)> {
        let code = std::fs::read_to_string(file).map_err(EngineError::Io)?;

        let path = Path::new(file);
        let file_stem = path
            .file_stem()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();
        let slug = file_stem.replace('_', "-");
        let language = Language::from_extension(
            path.extension()
                .and_then(|s| s.to_str())
                .unwrap_or_default(),
        );

        println!("🔍 Resolving ID for '{}'...", slug);
        let question = self.client.get_question_by_slug(&slug).await.map_err(|e| {
            EngineError::Other(format!(
                "Failed to fetch question. Does the filename match the problem slug? {}",
                e
            ))
        })?;

        Ok((code, slug, language, question))
    }

    /// Submits `file` as either a full submission or a test run and returns
    /// a structured [`SubmissionResult`].
    pub async fn submit_or_test(&self, file: &str, is_test: bool) -> Result<SubmissionResult> {
        let (code, slug, language, question) = self.read_and_resolve(file).await?;

        println!("🚀 Submitting {}...", file);

        if is_test {
            // NOTE: LeetCode's interpret_solution endpoint requires the internal `question_id`
            // (not the public `question_frontend_id`) in its request payload.
            let interpret_id = self
                .client
                .test_code(
                    &slug,
                    &question.question_id,
                    language.to_lang_slug(),
                    &code,
                    &question.example_test_cases,
                )
                .await
                .map_err(|e| EngineError::Other(format!("Test Submission failed: {}", e)))?;

            println!("⏳ Code queued. Waiting for execution results...");
            let r = self
                .client
                .check_test_submission(interpret_id)
                .await
                .map_err(|e| {
                    EngineError::Other(format!("Failed to check test submission status: {}", e))
                })?;

            let status_msg = r.status_msg.as_deref().unwrap_or("Unknown");
            Ok(SubmissionResult {
                status: SubmissionStatus::from_msg(status_msg),
                is_test: true,
                correct_answer: r.correct_answer,
                code_answer: r.code_answer,
                expected_code_answer: r.expected_code_answer,
                total_correct: r.total_correct,
                total_testcases: r.total_testcases,
                runtime: r.status_runtime,
                memory: r.status_memory,
                memory_percentile: r.memory_percentile,
                runtime_percentile: r.runtime_percentile,
                compile_error: None,
                full_runtime_error: r.full_runtime_error,
                input: None,
                expected_output: None,
                code_output: r.code_output.map(|v| v.join("\t")),
            })
        } else {
            // NOTE: LeetCode's submit endpoint requires the internal `question_id`
            // (not the public `question_frontend_id`) in its request payload.
            let submission_id = self
                .client
                .submit_code(&slug, &question.question_id, language.to_lang_slug(), &code)
                .await
                .map_err(|e| EngineError::Other(format!("Submission failed: {}", e)))?;

            println!("⏳ Code queued. Waiting for execution results...");
            let r = self
                .client
                .check_submission(submission_id)
                .await
                .map_err(|e| {
                    EngineError::Other(format!("Failed to check submission status: {}", e))
                })?;

            let status_msg = r.status_msg.as_deref().unwrap_or("Unknown");
            Ok(SubmissionResult {
                status: SubmissionStatus::from_msg(status_msg),
                is_test: false,
                correct_answer: None,
                code_answer: None,
                expected_code_answer: None,
                total_correct: r.total_correct,
                total_testcases: r.total_testcases,
                runtime: r.status_runtime,
                memory: r.status_memory,
                memory_percentile: r.memory_percentile,
                runtime_percentile: r.runtime_percentile,
                compile_error: r.compile_error,
                full_runtime_error: r.full_runtime_error,
                input: r.input,
                expected_output: r.expected_output,
                code_output: r.code_output,
            })
        }
    }
}
