//! `leetrs` — binary entry point.
//!
//! Parses CLI arguments with Clap and dispatches to the appropriate handler.
//! All heavy lifting (HTTP, TUI, file I/O) lives in the library crate under
//! the [`leetrs::commands`] module.
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use leetrs::{
    commands,
    config::{CONFIG, Config},
    models::{Identifier, Language},
};

#[derive(Parser, Debug)]
#[command(name = "leetrs")]
#[command(about = "A Neovim-integrated LeetCode TUI", long_about = None)]
#[command(version = option_env!("LEETRS_VERSION").unwrap_or(env!("CARGO_PKG_VERSION")))]
struct Cli {
    /// Enable verbose diagnostic logging
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Authenticate with LeetCode
    Auth,
    /// Launch the TUI
    Tui { language: Option<Language> },
    /// Check auth status
    Status,
    /// Pick a problem
    Pick {
        #[arg(value_parser = parse_identifier)]
        identifier: Identifier,
        language: Option<Language>,
        /// Print the problem description to stdout instead of opening the editor
        #[arg(short, long)]
        preview: bool,
        /// Open the problem in default web browser
        #[arg(short, long)]
        browser: bool,
    },
    /// Pick today's LeetCode Problem of the Day
    Daily {
        /// Language for the code template (defaults to config.toml)
        language: Option<Language>,
        /// Print the problem description to stdout instead of opening the editor
        #[arg(short, long)]
        preview: bool,
        /// Open the problem in default web browser
        #[arg(short, long)]
        browser: bool,
    },
    /// Submit a problem to leetcode
    Submit {
        /// The path to your solution file (e.g., 'two_sum.rs')
        file: String,
    },
    /// Test a problem without full submit
    Test {
        /// The path to your solution file (e.g., 'two_sum.rs')
        file: String,
        /// Custom test case input, one argument per line (a literal `\n` outside quotes also separates lines)
        #[arg(short, long, conflicts_with = "cases_file")]
        cases: Option<String>,
        /// Path to a file containing custom test case input
        #[arg(long)]
        cases_file: Option<PathBuf>,
    },
    /// Setup autocomplete for shell
    Completion { shell: clap_complete::Shell },
}

/// Parses a CLI identifier argument as either a numeric problem ID or a slug string.
fn parse_identifier(s: &str) -> Result<Identifier, String> {
    if let Ok(num) = s.parse::<u64>() {
        Ok(Identifier::Number(num))
    } else {
        Ok(Identifier::String(s.to_string()))
    }
}

#[tokio::main]
async fn main() -> std::process::ExitCode {
    let cli = Cli::parse();

    let mut builder = env_logger::Builder::from_default_env();
    if cli.verbose {
        builder.filter_level(log::LevelFilter::Debug);
    }
    let _ = builder.try_init();

    let config = match Config::new() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("❌ Error parsing config file: {}", e);
            return std::process::ExitCode::FAILURE;
        }
    };
    if let Err(e) = CONFIG.set(config) {
        eprintln!("❌ Config already initialized: {:?}", e);
        return std::process::ExitCode::FAILURE;
    }

    match run_command(cli).await {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("❌ {}", e);
            std::process::ExitCode::FAILURE
        }
    }
}

pub(crate) async fn run_command(cli: Cli) -> std::result::Result<(), String> {
    match cli.command {
        Some(Commands::Auth) => commands::handle_auth(),
        Some(Commands::Tui { language }) => commands::open_tui(&language).await,
        Some(Commands::Status) => commands::handle_status(),
        Some(Commands::Daily {
            language,
            preview,
            browser,
        }) => commands::handle_daily(&language, preview, browser).await,
        Some(Commands::Pick {
            identifier,
            language,
            preview,
            browser,
        }) => commands::handle_pick(&identifier, &language, preview, browser).await,
        Some(Commands::Test {
            file,
            cases,
            cases_file,
        }) => commands::handle_test(&file, cases.as_deref(), cases_file.as_deref()).await,
        Some(Commands::Submit { file }) => commands::handle_submit(&file).await,
        Some(Commands::Completion { shell }) => commands::handle_completion::<Cli>(&shell),
        None => commands::open_tui(&None).await,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_definition_is_valid() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }

    #[test]
    fn test_version_format() {
        use clap::CommandFactory;
        let version = Cli::command().render_version();
        let version = version.trim();
        let base = concat!("leetrs ", env!("CARGO_PKG_VERSION"));
        let unreleased_prefix = format!("{base}+");

        // The version must either match "leetrs <PKG_VERSION>" (when tagged/released)
        // or start with "leetrs <PKG_VERSION>+<commit>" (when untagged/unreleased).
        assert!(
            version == base || version.starts_with(&unreleased_prefix),
            "expected version '{version}' to be either '{base}' or start with '{unreleased_prefix}'"
        );
    }

    #[test]
    fn test_parse_test_with_inline_cases() {
        let cli =
            Cli::try_parse_from(["leetrs", "test", "two_sum.rs", "-c", "[3,2,4]\\n6"]).unwrap();
        match cli.command {
            Some(Commands::Test {
                file,
                cases,
                cases_file,
            }) => {
                assert_eq!(file, "two_sum.rs");
                assert_eq!(cases.as_deref(), Some("[3,2,4]\\n6"));
                assert!(cases_file.is_none());
            }
            other => panic!("unexpected command: {:?}", other),
        }
    }

    #[test]
    fn test_parse_test_rejects_both_case_sources() {
        let err = Cli::try_parse_from([
            "leetrs",
            "test",
            "two_sum.rs",
            "-c",
            "6",
            "--cases-file",
            "inputs.txt",
        ])
        .unwrap_err();
        assert_eq!(err.kind(), clap::error::ErrorKind::ArgumentConflict);
    }

    #[test]
    fn test_parse_daily_defaults() {
        let cli = Cli::try_parse_from(["leetrs", "daily"]).unwrap();
        match cli.command {
            Some(Commands::Daily {
                language,
                preview,
                browser,
            }) => {
                assert!(language.is_none());
                assert!(!preview);
                assert!(!browser);
            }
            other => panic!("unexpected command: {:?}", other),
        }
    }

    #[test]
    fn test_parse_daily_with_language_and_preview() {
        let cli = Cli::try_parse_from(["leetrs", "daily", "rust", "-p"]).unwrap();
        match cli.command {
            Some(Commands::Daily {
                language,
                preview,
                browser,
            }) => {
                assert_eq!(language, Some(Language::Rust));
                assert!(preview);
                assert!(!browser);
            }
            other => panic!("unexpected command: {:?}", other),
        }
    }

    #[test]
    fn test_parse_daily_rejects_flag_language() {
        let err = Cli::try_parse_from(["leetrs", "daily", "-l", "python"]).unwrap_err();
        assert_eq!(err.kind(), clap::error::ErrorKind::UnknownArgument);
    }

    #[test]
    fn test_parse_daily_with_browser() {
        let cli = Cli::try_parse_from(["leetrs", "daily", "-b"]).unwrap();
        match cli.command {
            Some(Commands::Daily { browser, .. }) => {
                assert!(browser);
            }
            other => panic!("unexpected command: {:?}", other),
        }
    }

    #[test]
    fn test_parse_daily_with_language_preview_and_browser() {
        let cli = Cli::try_parse_from(["leetrs", "daily", "rust", "-p", "-b"]).unwrap();
        match cli.command {
            Some(Commands::Daily {
                language,
                preview,
                browser,
            }) => {
                assert_eq!(language, Some(Language::Rust));
                assert!(preview);
                assert!(browser);
            }
            other => panic!("unexpected command: {:?}", other),
        }
    }

    #[test]
    fn test_parse_pick_with_browser() {
        let cli = Cli::try_parse_from(["leetrs", "pick", "1", "--browser"]).unwrap();
        match cli.command {
            Some(Commands::Pick { browser, .. }) => {
                assert!(browser);
            }
            other => panic!("unexpected command: {:?}", other),
        }
    }

    #[test]
    fn test_parse_pick_slug_with_browser_and_preview() {
        let cli = Cli::try_parse_from(["leetrs", "pick", "two-sum", "-b", "-p"]).unwrap();
        match cli.command {
            Some(Commands::Pick {
                identifier,
                preview,
                browser,
                ..
            }) => {
                assert_eq!(identifier, Identifier::String("two-sum".to_string()));
                assert!(preview);
                assert!(browser);
            }
            other => panic!("unexpected command: {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_run_command_test_nonexistent_file_returns_err() {
        let cli = Cli {
            verbose: false,
            command: Some(Commands::Test {
                file: "definitely_nonexistent_file_xyz123.rs".to_string(),
                cases: None,
                cases_file: None,
            }),
        };
        let res = run_command(cli).await;
        assert!(
            res.is_err(),
            "run_command for Test with missing file should return Err"
        );
    }

    #[tokio::test]
    async fn test_run_command_submit_nonexistent_file_returns_err() {
        let cli = Cli {
            verbose: false,
            command: Some(Commands::Submit {
                file: "definitely_nonexistent_file_xyz123.rs".to_string(),
            }),
        };
        let res = run_command(cli).await;
        assert!(
            res.is_err(),
            "run_command for Submit with missing file should return Err"
        );
    }
}
