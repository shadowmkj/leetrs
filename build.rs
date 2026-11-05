use std::process::Command;

fn main() {
    let pkg_version = env!("CARGO_PKG_VERSION");

    rerun_if_git_changes();

    let version = match get_unreleased_git_info() {
        Some(commit_hash) => format!("{pkg_version}+{commit_hash}"),
        None => pkg_version.to_string(),
    };

    println!("cargo:rustc-env=LEETRS_VERSION={version}");
}

/// Returns Git commit hash if this is an unreleased build (HEAD is not tagged).
/// Returns None if HEAD points to a tag or if git is unavailable.
fn get_unreleased_git_info() -> Option<String> {
    // Check if HEAD has any tag pointing directly to it
    let has_tag = Command::new("git")
        .args(["tag", "--points-at", "HEAD"])
        .output()
        .map(|out| {
            out.status.success()
                && String::from_utf8_lossy(&out.stdout)
                    .lines()
                    .any(|line| !line.trim().is_empty())
        })
        .unwrap_or(false);

    // If HEAD is tagged, this is a released version (do not embed commit hash)
    if has_tag {
        return None;
    }

    // Otherwise, this is an unreleased build: retrieve the short commit hash
    Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .filter(|out| out.status.success())
        .map(|out| String::from_utf8_lossy(&out.stdout).trim().to_string())
}

/// Watches git reference files so Cargo rebuilds when branches, commits, or tags change.
fn rerun_if_git_changes() {
    let git_dir = if std::fs::metadata(".git")
        .map(|m| m.is_file())
        .unwrap_or(false)
    {
        // Worktree or submodule: .git is a file containing "gitdir: <path>"
        std::fs::read_to_string(".git")
            .ok()
            .and_then(|content| {
                content
                    .strip_prefix("gitdir:")
                    .map(|p| std::path::PathBuf::from(p.trim()))
            })
            .unwrap_or_else(|| std::path::PathBuf::from(".git"))
    } else {
        std::path::PathBuf::from(".git")
    };

    if !git_dir.exists() {
        return;
    }

    println!("cargo:rerun-if-changed={}", git_dir.join("index").display());
    println!("cargo:rerun-if-changed={}", git_dir.join("HEAD").display());
    println!(
        "cargo:rerun-if-changed={}",
        git_dir.join("refs/tags").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        git_dir.join("packed-refs").display()
    );

    if let Ok(head) = std::fs::read_to_string(git_dir.join("HEAD"))
        && let Some(ref_path) = head.strip_prefix("ref:")
    {
        let ref_file = git_dir.join(ref_path.trim());
        println!("cargo:rerun-if-changed={}", ref_file.display());
    }
}
