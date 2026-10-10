//! `xtask` — developer and CI automation tasks for `leetrs`.
//!
//! Provides a standardized CLI (`cargo xtask <subcommand>`) for linting,
//! testing, coverage, hook installation, and workspace hygiene.
use std::{
    path::{Path, PathBuf},
    process::{Command, ExitStatus},
};

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "xtask")]
#[command(about = "Task runner for leetrs development")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Runs all pre-commit / CI quality gates (format check, clippy, tests).
    Ci,
    /// Formats all workspace files using rustfmt.
    Fmt {
        /// Check formatting without modifying files.
        #[arg(long)]
        check: bool,
    },
    /// Runs Clippy across all targets with warnings denied.
    Clippy,
    /// Runs all unit tests.
    Test,
    /// Generates code coverage report using cargo-llvm-cov.
    Coverage {
        /// Open interactive HTML report in browser.
        #[arg(long)]
        html: bool,
        /// Generate LCOV report for CI/Codecov upload.
        #[arg(long)]
        lcov: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let root = project_root()?;

    match cli.command {
        Commands::Ci => run_ci(&root),
        Commands::Fmt { check } => run_fmt(&root, check),
        Commands::Clippy => run_clippy(&root),
        Commands::Test => run_test(&root),
        Commands::Coverage { html, lcov } => run_coverage(&root, html, lcov),
    }
}

// ==============================================================================
// Task Implementations
// ==============================================================================

fn run_ci(root: &Path) -> Result<()> {
    println!("🚀 Running full CI quality gate...\n");

    println!("🔍 1/3: Checking formatting...");
    run_fmt(root, true)?;

    println!("🔍 2/3: Running Clippy (-D warnings)...");
    run_clippy(root)?;

    println!("🧪 3/3: Running unit tests...");
    run_test(root)?;

    println!("\n✅ All quality gates passed!");
    Ok(())
}

fn run_fmt(root: &Path, check: bool) -> Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.current_dir(root).arg("fmt").arg("--all");
    if check {
        cmd.arg("--check");
    }
    execute_command(&mut cmd, "while checking/running code formatting")
}

fn run_clippy(root: &Path) -> Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.current_dir(root)
        .arg("clippy")
        .arg("--all-targets")
        .arg("--")
        .arg("-D")
        .arg("warnings");
    execute_command(&mut cmd, "while running clippy lint analysis")
}

fn run_test(root: &Path) -> Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.current_dir(root).arg("test").arg("--workspace");
    execute_command(&mut cmd, "while running test suite")
}

fn run_coverage(root: &Path, html: bool, lcov: bool) -> Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.current_dir(root)
        .arg("llvm-cov")
        .arg("--all-features")
        .arg("--workspace");

    if html {
        cmd.arg("--html");
    } else if lcov {
        cmd.arg("--lcov").arg("--output-path").arg("lcov.info");
    }

    execute_command(&mut cmd, "while running code coverage analysis")
}

// ==============================================================================
// Helper Utilities
// ==============================================================================

fn execute_command(cmd: &mut Command, context_msg: &'static str) -> Result<()> {
    let status: ExitStatus = cmd.status().context(context_msg)?;

    if !status.success() {
        bail!("Command failed with exit status: {}", status);
    }
    Ok(())
}

fn project_root() -> Result<PathBuf> {
    let manifest_dir =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string()));

    if manifest_dir.ends_with("xtask") {
        Ok(manifest_dir
            .parent()
            .expect("xtask directory must have a parent directory")
            .to_path_buf())
    } else {
        Ok(manifest_dir)
    }
}
