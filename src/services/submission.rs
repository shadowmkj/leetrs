//! Submission service — orchestrates the submit/test workflow.
//!
//! Extracts the shared logic between [`Picker::test_submit`] and
//! [`Picker::submit`] into a single `submit_or_test` method with a structured
//! result type that callers can pattern-match on.
use std::path::{Path, PathBuf};

use crate::config::solutions_dir;

use crate::{
    client::LeetCodeClient,
    error::{EngineError, Result},
    models::{Language, Question, SubmissionCheckResult, TestSubmissionCheckResult},
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
    pub std_output_list: Option<Vec<String>>,
    // Submit-run specific
    pub input: Option<String>,
    pub expected_output: Option<String>,
    pub code_output: Option<String>,
}

impl SubmissionResult {
    /// Constructs a [`SubmissionResult`] from the polled response of a test execution (`interpret_solution`).
    pub fn from_test_check(r: TestSubmissionCheckResult) -> Self {
        let status_msg = r.status_msg.as_deref().unwrap_or("Unknown");
        let std_output_list = r
            .std_output_list
            .filter(|list| !list.is_empty())
            .or_else(|| r.std_output.map(|s| vec![s]));
        Self {
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
            std_output_list,
        }
    }

    /// Constructs a [`SubmissionResult`] from the polled response of a full submission (`submit`).
    pub fn from_submit_check(r: SubmissionCheckResult) -> Self {
        let status_msg = r.status_msg.as_deref().unwrap_or("Unknown");
        let std_output_list = r
            .std_output_list
            .filter(|list| !list.is_empty())
            .or_else(|| r.std_output.map(|s| vec![s]));
        Self {
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
            std_output_list,
        }
    }
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
        let dir = solutions_dir();
        let path = resolve_solution_file(file, &dir);
        let code = std::fs::read_to_string(&path).map_err(|e| {
            // Only name solutions_dir when it was actually searched.
            let also = if dir.as_os_str().is_empty() || Path::new(file).is_absolute() {
                String::new()
            } else {
                format!(" (also looked in {})", dir.display())
            };
            EngineError::Io(std::io::Error::new(
                e.kind(),
                format!("could not read '{}'{}: {}", file, also, e),
            ))
        })?;

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
    ///
    /// Test runs use `custom_cases` as the judge input when given, otherwise
    /// the problem's example test cases. Full submissions ignore it.
    pub async fn submit_or_test(
        &self,
        file: &str,
        is_test: bool,
        custom_cases: Option<&str>,
    ) -> Result<SubmissionResult> {
        let (code, slug, language, question) = self.read_and_resolve(file).await?;

        println!("🚀 Submitting {}...", file);

        if is_test {
            if custom_cases.is_some() {
                println!("🧪 Running against custom test cases");
            }
            // NOTE: LeetCode's interpret_solution endpoint requires the internal `question_id`
            // (not the public `question_frontend_id`) in its request payload.
            let interpret_id = self
                .client
                .test_code(
                    &slug,
                    &question.question_id,
                    language.to_lang_slug(),
                    &code,
                    custom_cases.unwrap_or(&question.example_test_cases),
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

            Ok(SubmissionResult::from_test_check(r))
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

            Ok(SubmissionResult::from_submit_check(r))
        }
    }
}

/// `file` as given if it exists, otherwise the same name under the configured
/// `solutions_dir` (which `pick` writes to). A file found in neither place
/// resolves to `file` as typed.
fn resolve_solution_file(file: &str, solutions_dir: &Path) -> PathBuf {
    let path = PathBuf::from(file);
    if path.exists() {
        return path;
    }
    let in_solutions_dir = solutions_dir.join(&path);
    if in_solutions_dir.exists() {
        in_solutions_dir
    } else {
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::LeetCodeCredentials;

    #[test]
    fn test_submission_status_from_msg() {
        assert_eq!(
            SubmissionStatus::from_msg("Accepted"),
            SubmissionStatus::Accepted
        );
        assert_eq!(
            SubmissionStatus::from_msg("Wrong Answer"),
            SubmissionStatus::WrongAnswer
        );
        assert_eq!(
            SubmissionStatus::from_msg("Compile Error"),
            SubmissionStatus::CompileError
        );
        assert_eq!(
            SubmissionStatus::from_msg("Runtime Error"),
            SubmissionStatus::RuntimeError
        );
        assert_eq!(
            SubmissionStatus::from_msg("Time Limit Exceeded"),
            SubmissionStatus::TimeLimitExceeded
        );
        assert_eq!(
            SubmissionStatus::from_msg("Memory Limit Exceeded"),
            SubmissionStatus::Unknown("Memory Limit Exceeded".to_string())
        );
    }

    #[test]
    fn test_from_test_check_with_std_output_list() {
        let check = TestSubmissionCheckResult {
            code_answer: Some(vec!["[0, 1]".into()]),
            code_output: Some(vec!["out1".into(), "out2".into()]),
            correct_answer: Some(true),
            expected_code_answer: Some(vec!["[0, 1]".into()]),
            full_runtime_error: None,
            lang: Some("rust".into()),
            memory_percentile: Some(85.0),
            run_success: Some(true),
            runtime_percentile: Some(92.0),
            state: "SUCCESS".into(),
            status_memory: Some("2 MB".into()),
            status_msg: Some("Accepted".into()),
            status_runtime: Some("0 ms".into()),
            std_output: Some("ignored fallback".into()),
            std_output_list: Some(vec!["case 1 debug\n".into(), "case 2 debug\n".into()]),
            total_correct: Some(2),
            total_testcases: Some(2),
        };

        let result = SubmissionResult::from_test_check(check);
        assert!(result.is_test);
        assert_eq!(result.status, SubmissionStatus::Accepted);
        assert_eq!(result.correct_answer, Some(true));
        assert_eq!(result.code_output.as_deref(), Some("out1\tout2"));
        assert_eq!(
            result.std_output_list,
            Some(vec!["case 1 debug\n".into(), "case 2 debug\n".into()])
        );
        assert_eq!(result.runtime.as_deref(), Some("0 ms"));
        assert_eq!(result.memory.as_deref(), Some("2 MB"));
    }

    #[test]
    fn test_from_test_check_fallback_on_empty_list() {
        let check = TestSubmissionCheckResult {
            code_answer: None,
            code_output: None,
            correct_answer: Some(false),
            expected_code_answer: None,
            full_runtime_error: Some("panic message".into()),
            lang: Some("rust".into()),
            memory_percentile: None,
            run_success: Some(false),
            runtime_percentile: None,
            state: "SUCCESS".into(),
            status_memory: None,
            status_msg: Some("Runtime Error".into()),
            status_runtime: None,
            std_output: Some("single fallback output\n".into()),
            std_output_list: Some(vec![]),
            total_correct: Some(0),
            total_testcases: Some(1),
        };

        let result = SubmissionResult::from_test_check(check);
        assert!(result.is_test);
        assert_eq!(result.status, SubmissionStatus::RuntimeError);
        assert_eq!(result.full_runtime_error.as_deref(), Some("panic message"));
        assert_eq!(
            result.std_output_list,
            Some(vec!["single fallback output\n".into()])
        );
    }

    #[test]
    fn test_from_test_check_none_stdout_and_default_status() {
        let check = TestSubmissionCheckResult {
            code_answer: None,
            code_output: None,
            correct_answer: None,
            expected_code_answer: None,
            full_runtime_error: None,
            lang: None,
            memory_percentile: None,
            run_success: None,
            runtime_percentile: None,
            state: "SUCCESS".into(),
            status_memory: None,
            status_msg: None, // should default to "Unknown"
            status_runtime: None,
            std_output: None,
            std_output_list: None,
            total_correct: None,
            total_testcases: None,
        };

        let result = SubmissionResult::from_test_check(check);
        assert_eq!(result.status, SubmissionStatus::Unknown("Unknown".into()));
        assert_eq!(result.std_output_list, None);
    }

    #[test]
    fn test_from_submit_check_with_std_output_and_fields() {
        let check = SubmissionCheckResult {
            code_output: Some("[3,0]".into()),
            compile_error: Some("unused variable".into()),
            expected_output: Some("[0,1]".into()),
            finished: Some(true),
            full_runtime_error: None,
            input: Some("[2,7,11,15]\n9".into()),
            input_formatted: None,
            last_testcase: Some("[2,7,11,15]\n9".into()),
            memory_percentile: Some(45.0),
            run_success: Some(false),
            runtime_percentile: Some(50.0),
            state: "SUCCESS".into(),
            status_memory: Some("12 MB".into()),
            status_msg: Some("Wrong Answer".into()),
            status_runtime: Some("4 ms".into()),
            std_output: Some("submit log\n".into()),
            std_output_list: None,
            total_correct: Some(12),
            total_testcases: Some(57),
        };

        let result = SubmissionResult::from_submit_check(check);
        assert!(!result.is_test);
        assert_eq!(result.status, SubmissionStatus::WrongAnswer);
        assert_eq!(result.input.as_deref(), Some("[2,7,11,15]\n9"));
        assert_eq!(result.expected_output.as_deref(), Some("[0,1]"));
        assert_eq!(result.code_output.as_deref(), Some("[3,0]"));
        assert_eq!(result.compile_error.as_deref(), Some("unused variable"));
        assert_eq!(result.std_output_list, Some(vec!["submit log\n".into()]));
    }

    #[test]
    fn test_from_submit_check_std_output_list_precedence() {
        let check = SubmissionCheckResult {
            code_output: None,
            compile_error: None,
            expected_output: None,
            finished: Some(true),
            full_runtime_error: None,
            input: None,
            input_formatted: None,
            last_testcase: None,
            memory_percentile: None,
            run_success: Some(true),
            runtime_percentile: None,
            state: "SUCCESS".into(),
            status_memory: None,
            status_msg: Some("Accepted".into()),
            status_runtime: None,
            std_output: Some("ignored single".into()),
            std_output_list: Some(vec!["preferred list\n".into()]),
            total_correct: Some(57),
            total_testcases: Some(57),
        };

        let result = SubmissionResult::from_submit_check(check);
        assert_eq!(
            result.std_output_list,
            Some(vec!["preferred list\n".into()])
        );
    }

    #[test]
    fn resolve_solution_file_falls_back_to_solutions_dir() {
        let dir = std::env::temp_dir().join(format!("leetrs-solutions-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let name = "leetrs_resolve_fallback_two_sum.rs";
        std::fs::write(dir.join(name), "").unwrap();

        // Not in the current directory: found under solutions_dir.
        assert_eq!(resolve_solution_file(name, &dir), dir.join(name));
        // Missing everywhere: the path as typed, so the error names it.
        assert_eq!(
            resolve_solution_file("leetrs_missing_xyz.rs", &dir),
            PathBuf::from("leetrs_missing_xyz.rs")
        );
        // A relative name in the current directory wins over solutions_dir.
        assert_eq!(
            resolve_solution_file("Cargo.toml", &dir),
            PathBuf::from("Cargo.toml")
        );
        // An existing path wins over solutions_dir.
        let here = dir.join(name);
        let here = here.to_str().unwrap();
        assert_eq!(
            resolve_solution_file(here, Path::new("/nonexistent")),
            PathBuf::from(here)
        );

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[tokio::test]
    async fn test_submission_service_file_not_found() {
        let creds = LeetCodeCredentials {
            session_cookie: "dummy_session".to_string(),
            csrf_token: "dummy_csrf".to_string(),
        };
        let client = LeetCodeClient::new(creds).expect("client creation failed");
        let service = SubmissionService::new(client);

        let err = service
            .submit_or_test("non_existent_problem_file_xyz.rs", true, None)
            .await;
        assert!(matches!(err, Err(EngineError::Io(_))));
    }
}
