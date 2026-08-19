//! `leetrs` — binary entry point.
//!
//! Parses CLI arguments with Clap and dispatches to the appropriate handler.
//! All heavy lifting (HTTP, TUI, file I/O) lives in the library crate under
//! the [`leetrs::commands`] module.
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
    /// Submit a problem to leetcode
    Submit {
        /// The path to your solution file (e.g., 'two_sum.rs')
        file: String,
    },
    /// Test a problem without full submit
    Test {
        /// The path to your solution file (e.g., 'two_sum.rs')
        file: String,
    },
    /// Setup autocomplete for shell
    Completion { shell: clap_complete::Shell },
}

/// Parses a CLI identifier argument as either a numeric problem ID or a slug string.
fn parse_identifier(s: &str) -> Result<Identifier, String> {
    Ok(match s.parse::<u64>() {
        Ok(num) => Identifier::Number(num),
        Err(_) => Identifier::String(s.to_string()),
    })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config = Config::new().expect("Error parsing config file");
    CONFIG.set(config).expect("Config already initialized");

    match &cli.command {
        Some(Commands::Auth) => commands::handle_auth(),
        Some(Commands::Tui { language }) => commands::open_tui(language).await,
        Some(Commands::Status) => commands::handle_status(),
        Some(Commands::Pick {
            identifier,
            language,
            preview,
        }) => {
            if let Err(e) = commands::handle_pick(identifier, language, *preview).await {
                eprintln!("❌ {}", e);
            }
        }
        Some(Commands::Test { file }) => {
            if let Err(e) = commands::handle_test(file).await {
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
