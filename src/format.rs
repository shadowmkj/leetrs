//! CLI result formatting for submission and test results.
//!
//! Keeps all `println!` / `eprintln!` calls out of business logic so that
//! [`SubmissionResult`] can later be displayed in the TUI as well.
use crate::services::submission::{SubmissionResult, SubmissionStatus};

/// Prints a human-readable summary of a submission or test result to stdout.
pub fn format_result(result: &SubmissionResult) {
    println!("\n==================================================");

    if result.is_test {
        let passed = result.correct_answer.unwrap_or(false);
        if passed {
            println!("  ✅ All test cases passed");
        } else {
            println!("  ❌ Test Failed");
        }
        println!("==================================================\n");

        if let (Some(correct), Some(total)) = (result.total_correct, result.total_testcases) {
            println!("🧪 Testcases: {} / {} passed", correct, total);
        }

        match &result.status {
            SubmissionStatus::Accepted if passed => {
                print_performance_metrics(result);
            }
            SubmissionStatus::Accepted => {
                if let (Some(code_answers), Some(expected)) =
                    (&result.code_answer, &result.expected_code_answer)
                {
                    println!("Expected");
                    println!("{}", expected.join("\t"));
                    println!("Output");
                    println!("{}", code_answers.join("\t"));
                }
            }
            SubmissionStatus::RuntimeError => {
                if let Some(err) = &result.full_runtime_error {
                    println!("❌ Error\n{}", err);
                }
            }
            _ => {}
        }
    } else {
        match &result.status {
            SubmissionStatus::Accepted => println!("  ✅ Accepted"),
            SubmissionStatus::WrongAnswer => println!("  ❌ Wrong Answer"),
            SubmissionStatus::CompileError => println!("  ❌ Compile Error"),
            SubmissionStatus::RuntimeError => println!("  ❌ Runtime Error"),
            SubmissionStatus::TimeLimitExceeded => println!("  ❌ Time Limit Exceeded"),
            SubmissionStatus::Unknown(msg) => println!("  ❌ {}", msg),
        }
        println!("==================================================\n");

        if let (Some(correct), Some(total)) = (result.total_correct, result.total_testcases) {
            println!("🧪 Testcases: {} / {} passed", correct, total);
        }

        match &result.status {
            SubmissionStatus::Accepted => {
                print_performance_metrics(result);
            }
            SubmissionStatus::CompileError => {
                if let Some(err) = &result.compile_error {
                    println!("💥 Compiler Output:\n{}", err);
                }
            }
            SubmissionStatus::WrongAnswer => {
                if let Some(input) = &result.input {
                    print!("INPUT: ");
                    for part in input.split('\n') {
                        print!("{}\t", part);
                    }
                    println!();
                }
                if let (Some(expected), Some(output)) =
                    (&result.expected_output, &result.code_output)
                {
                    println!("Expected: {}\nOutput: {}", expected, output);
                }
            }
            SubmissionStatus::RuntimeError => {
                if let Some(err) = &result.full_runtime_error {
                    println!("❌ Error\n{}", err);
                }
            }
            _ => {}
        }
    }
}

fn print_performance_metrics(result: &SubmissionResult) {
    if let Some(runtime) = &result.runtime {
        println!("⏱️ Runtime: {}", runtime);
    }
    if let Some(memory) = &result.memory {
        println!("💾 Memory: {}", memory);
    }
    if let Some(mp) = result.memory_percentile {
        println!("📝 Memory Percentile: {:.2}%", mp);
    }
    if let Some(rp) = result.runtime_percentile {
        println!("⏰ Runtime Percentile: {:.2}%", rp);
    }
}
