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
    /// Runs all pre-commit / CI quality gates (format check, clippy across host & cross targets, tests).
    Ci {
        /// Optional specific target triple to check (e.g. x86_64-unknown-linux-gnu).
        #[arg(long)]
        target: Option<String>,
    },
    /// Formats all workspace files using rustfmt.
    Fmt {
        /// Check formatting without modifying files.
        #[arg(long)]
        check: bool,
    },
    /// Runs Clippy across all targets with warnings denied.
    Clippy {
        /// Optional specific target triple to check (e.g. x86_64-unknown-linux-gnu).
        #[arg(long)]
        target: Option<String>,
        /// Do not check installed cross-compilation targets.
        #[arg(long)]
        no_cross: bool,
    },
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
        Commands::Ci { target } => run_ci(&root, target.as_deref()),
        Commands::Fmt { check } => run_fmt(&root, check),
        Commands::Clippy { target, no_cross } => run_clippy(&root, target.as_deref(), !no_cross),
        Commands::Test => run_test(&root),
        Commands::Coverage { html, lcov } => run_coverage(&root, html, lcov),
    }
}

// ==============================================================================
// Task Implementations
// ==============================================================================

fn run_ci(root: &Path, target: Option<&str>) -> Result<()> {
    println!("🚀 Running full CI quality gate...\n");

    println!("🔍 1/3: Checking formatting...");
    run_fmt(root, true)?;

    println!("🔍 2/3: Running Clippy (-D warnings)...");
    run_clippy(root, target, true)?;

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

fn run_clippy(root: &Path, target: Option<&str>, check_cross: bool) -> Result<()> {
    if let Some(tgt) = target {
        println!("  ⚙️ Target: {}...", tgt);
        let mut cmd = Command::new("cargo");
        cmd.current_dir(root)
            .arg("clippy")
            .arg("--workspace")
            .arg("--target")
            .arg(tgt)
            .arg("--all-targets")
            .arg("--")
            .arg("-D")
            .arg("warnings");
        return execute_command(&mut cmd, "while running clippy lint analysis");
    }

    println!("  ⚙️ Target: host (default)...");
    let mut cmd = Command::new("cargo");
    cmd.current_dir(root)
        .arg("clippy")
        .arg("--all-targets")
        .arg("--")
        .arg("-D")
        .arg("warnings");
    execute_command(&mut cmd, "while running clippy lint analysis")?;

    if check_cross {
        let host = get_host_target().unwrap_or_default();
        let installed = get_installed_targets();

        let supported_cross_targets = [
            "x86_64-unknown-linux-gnu",
            "aarch64-unknown-linux-gnu",
            "x86_64-apple-darwin",
            "aarch64-apple-darwin",
        ];

        let mut checked_any_cross = false;
        for tgt in supported_cross_targets {
            if tgt != host && installed.iter().any(|i| i == tgt) {
                println!("  ⚙️ Cross-checking target: {}...", tgt);
                let mut cross_cmd = Command::new("cargo");
                cross_cmd
                    .current_dir(root)
                    .arg("clippy")
                    .arg("--workspace")
                    .arg("--target")
                    .arg(tgt)
                    .arg("--all-targets")
                    .arg("--")
                    .arg("-D")
                    .arg("warnings");
                execute_command(&mut cross_cmd, "while running cross-target clippy analysis")?;
                checked_any_cross = true;
            }
        }

        if !checked_any_cross
            && !host.contains("linux")
            && !installed.iter().any(|i| i == "x86_64-unknown-linux-gnu")
        {
            println!(
                "💡 Tip: Run 'rustup target add x86_64-unknown-linux-gnu' to enable automated cross-platform Linux linting."
            );
        }
    }

    Ok(())
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

fn get_host_target() -> Option<String> {
    let output = Command::new("rustc").arg("-vV").output().ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if let Some(host) = line.strip_prefix("host: ") {
            return Some(host.trim().to_string());
        }
    }
    None
}

fn get_installed_targets() -> Vec<String> {
    if let Ok(output) = Command::new("rustup")
        .args(["target", "list", "--installed"])
        .output()
        && output.status.success()
    {
        return String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
    }
    Vec::new()
}
