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
#[command(version)]
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
        #[arg(short, long)]
        preview: bool,
    },
    /// Pick today's LeetCode Problem of the Day
    Daily {
        /// Language for the code template (defaults to config.toml)
        language: Option<Language>,
        /// Print the problem description to stdout instead of opening the editor
        #[arg(short, long)]
        preview: bool,
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
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let mut builder = env_logger::Builder::from_default_env();
    if cli.verbose {
        builder.filter_level(log::LevelFilter::Debug);
    }
    let _ = builder.try_init();

    let config = Config::new().expect("Error parsing config file");
    CONFIG.set(config).expect("Config already initialized");

    match &cli.command {
        Some(Commands::Auth) => commands::handle_auth(),
        Some(Commands::Tui { language }) => commands::open_tui(language).await,
        Some(Commands::Status) => commands::handle_status(),
        Some(Commands::Daily { language, preview }) => {
            if let Err(e) = commands::handle_daily(language, *preview).await {
                eprintln!("❌ {}", e);
            }
        }
        Some(Commands::Pick {
            identifier,
            language,
            preview,
        }) => {
            if let Err(e) = commands::handle_pick(identifier, language, *preview).await {
                eprintln!("❌ {}", e);
            }
        }
        Some(Commands::Test {
            file,
            cases,
            cases_file,
        }) => {
            if let Err(e) =
                commands::handle_test(file, cases.as_deref(), cases_file.as_deref()).await
            {
                eprintln!("❌ {}", e);
            }
        }
        Some(Commands::Submit { file }) => {
            if let Err(e) = commands::handle_submit(file).await {
                eprintln!("❌ {}", e);
            }
        }
        Some(Commands::Completion { shell }) => {
            commands::handle_completion::<Cli>(shell);
        }
        None => commands::open_tui(&None).await,
    };

    Ok(())
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
            Some(Commands::Daily { language, preview }) => {
                assert!(language.is_none());
                assert!(!preview);
            }
            other => panic!("unexpected command: {:?}", other),
        }
    }

    #[test]
    fn test_parse_daily_with_language_and_preview() {
        let cli = Cli::try_parse_from(["leetrs", "daily", "rust", "-p"]).unwrap();
        match cli.command {
            Some(Commands::Daily { language, preview }) => {
                assert_eq!(language, Some(Language::Rust));
                assert!(preview);
            }
            other => panic!("unexpected command: {:?}", other),
        }
    }

    #[test]
    fn test_parse_daily_rejects_flag_language() {
        let err = Cli::try_parse_from(["leetrs", "daily", "-l", "python"]).unwrap_err();
        assert_eq!(err.kind(), clap::error::ErrorKind::UnknownArgument);
    }
}
