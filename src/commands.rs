//! Command handlers extracted from `main.rs`.
//!
//! Each public function here corresponds to one CLI sub-command. Separating
//! them from `main.rs` keeps the binary entry-point thin (CLI parsing only)
//! and makes the handlers independently testable.
use std::{fs, io, path::Path, process::Command, rc::Rc};

use clap::CommandFactory;
use clap_complete::{
    Shell,
    aot::{Bash, Fish, Zsh},
    generate,
};
use dialoguer::{Select, theme::ColorfulTheme};

use crate::{
    auth::{LeetCodeCredentials, auto_extract_flow, manual_auth_flow},
    client::LeetCodeClient,
    config::CONFIG,
    models::{Identifier, Language, ProblemSummary},
    picker::Picker,
};

/// Builds a [`Picker`] from stored credentials, returning an error if the
/// user has not authenticated yet.
pub async fn make_picker() -> std::result::Result<Picker, String> {
    let creds = LeetCodeCredentials::load()
        .ok_or_else(|| "Not authenticated. Please run `leetrs auth` first.".to_string())?;
    let client =
        LeetCodeClient::new(creds).map_err(|e| format!("Failed to initialize client: {}", e))?;
    Ok(Picker::new(client))
}

/// Interactive authentication flow — prompts the user to choose a method.
pub fn handle_auth() {
    println!("🔒 LeetCode Authentication\n");

    let options = &[
        "Auto-detect browser (Recommended)",
        "Paste cURL / Cookie header manually",
        "Extract from Firefox",
        "Extract from Chrome",
        "Extract from Brave",
        "Extract from Edge",
        "Extract from Arc",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("How would you like to authenticate?")
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();

    let credentials_result = match selection {
        0 => auto_extract_flow("auto"),
        1 => manual_auth_flow(),
        2 => auto_extract_flow("firefox"),
        3 => auto_extract_flow("chrome"),
        4 => auto_extract_flow("brave"),
        5 => auto_extract_flow("edge"),
        6 => auto_extract_flow("arc"),
        _ => unreachable!(),
    };

    log::debug!(
        "handle_auth selection: {}, result: {:?}",
        selection,
        credentials_result.is_ok()
    );

    match credentials_result {
        Ok(creds) => match creds.save() {
            Ok(_) => println!("\n✅ Authentication successful!"),
            Err(e) => eprintln!("\n❌ Failed to save credentials: {}", e),
        },
        Err(e) => {
            eprintln!("\n❌ Authentication failed: {}", e);
            if selection != 0 {
                eprintln!(
                    "Tip: Make sure you are logged into leetcode.com on that browser, or try the manual option."
                );
            }
        }
    }
}

/// Prints current credential status to stdout.
pub fn handle_status() {
    match LeetCodeCredentials::load() {
        Some(creds) => {
            println!("✅ Currently authenticated!");
            println!("🔑 csrftoken:");
            println!("{}\n", creds.csrf_token);
            println!("🔑 LEETCODE_SESSION:");
            println!("{}", creds.session_cookie);
        }
        None => {
            eprintln!("❌ Not authenticated. No valid credentials found.");
            eprintln!("Run `leetrs auth` to set up your account.");
        }
    }
}

/// Resolves and writes the problem files, then opens Neovim with the description
/// and code side-by-side in a vertical split.
///
/// When `preview` is true the Markdown description is printed to stdout
/// instead of opening an editor.
pub async fn handle_pick(
    identifier: &Identifier,
    language: &Option<Language>,
    preview: bool,
) -> std::result::Result<(), String> {
    let picker = make_picker().await?;
    pick_and_open(&picker, identifier, language, preview).await
}

/// Fetches today's Problem of the Day and opens it the way `pick` does.
pub async fn handle_daily(
    language: &Option<Language>,
    preview: bool,
) -> std::result::Result<(), String> {
    let picker = make_picker().await?;
    let daily = picker
        .client
        .get_daily_challenge()
        .await
        .map_err(|e| format!("Failed to fetch today's daily challenge: {}", e))?;
    let question = &daily.question;
    // stderr, so `leetrs daily --preview > problem.md` keeps this line out.
    eprintln!(
        "📅 Daily challenge for {}: {}. {} ({})",
        daily.date, question.question_frontend_id, question.title, question.difficulty
    );

    if question.is_paid_only {
        let is_premium = picker
            .client
            .get_user_detail()
            .await
            .ok()
            .and_then(|user| user.is_premium);
        if let Some(warning) = premium_only_gate(is_premium)? {
            eprintln!("{}", warning);
        }
    }

    let identifier = Identifier::String(question.title_slug.clone());
    pick_and_open(&picker, &identifier, language, preview).await
}

/// Decides whether a premium-only daily problem can be picked.
///
/// LeetCode returns no description or code template for a premium problem to
/// a non-premium account, so picking it would fail on a missing field. Stop
/// with a clear message when the account is known not to be premium, and only
/// warn when the profile lookup could not say.
fn premium_only_gate(
    is_premium: Option<bool>,
) -> std::result::Result<Option<&'static str>, String> {
    match is_premium {
        Some(true) => Ok(None),
        Some(false) => Err(
            "Today's problem is for LeetCode Premium subscribers, and this account is not premium."
                .to_string(),
        ),
        None => Ok(Some(
            "⚠️ Today's problem is for LeetCode Premium subscribers and premium status could not be confirmed; fetching it may fail.",
        )),
    }
}

/// Runs the solution against example test cases (or custom input from
/// `cases` / `cases_file`) without recording a submission.
pub async fn handle_test(
    file: &str,
    cases: Option<&str>,
    cases_file: Option<&Path>,
) -> std::result::Result<(), String> {
    let custom_cases = resolve_custom_cases(cases, cases_file)?;
    let picker = make_picker().await?;
    picker.test_submit(file, custom_cases.as_deref()).await;
    Ok(())
}

/// Resolves custom test input from `--cases` or `--cases-file`.
///
/// Inline input treats a literal `\n` as a line break so `-c "[3,2,4]\n6"`
/// works in shells that don't expand escapes. Both sources then get CRLF
/// normalized and trailing whitespace trimmed so a final newline doesn't
/// become an empty test line. Empty input, or both sources at once, is rejected.
fn resolve_custom_cases(
    cases: Option<&str>,
    cases_file: Option<&Path>,
) -> std::result::Result<Option<String>, String> {
    if cases.is_some() && cases_file.is_some() {
        return Err("cannot specify both --cases and --cases-file".to_string());
    }
    let raw = if let Some(path) = cases_file {
        fs::read_to_string(path)
            .map_err(|e| format!("Failed to read cases file '{}': {}", path.display(), e))?
    } else if let Some(cases) = cases {
        unescape_line_breaks(cases)
    } else {
        return Ok(None);
    };
    let resolved = raw.replace("\r\n", "\n").trim_end().to_string();

    if resolved.trim().is_empty() {
        return Err("Custom test input is empty".to_string());
    }
    Ok(Some(resolved))
}

/// Replaces literal `\n` with a line break, except inside JSON string
/// literals, where `\n` is an escape the judge must see unchanged
/// (e.g. `"dir\n\tfile.ext"`).
fn unescape_line_breaks(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    let mut in_string = false;
    while let Some(c) = chars.next() {
        match c {
            '"' => {
                in_string = !in_string;
                out.push(c);
            }
            '\\' if in_string => {
                out.push(c);
                if let Some(escaped) = chars.next() {
                    out.push(escaped);
                }
            }
            '\\' if chars.peek() == Some(&'n') => {
                chars.next();
                out.push('\n');
            }
            _ => out.push(c),
        }
    }
    out
}

/// Submits the solution to LeetCode for full judging.
pub async fn handle_submit(file: &str) -> std::result::Result<(), String> {
    let picker = make_picker().await?;
    picker.submit(file).await;
    Ok(())
}

/// Writes shell completion script for `shell` to stdout.
pub fn handle_completion<C: CommandFactory>(shell: &Shell) {
    let mut cmd = C::command();
    match shell {
        Shell::Bash => generate(Bash, &mut cmd, "leetrs", &mut io::stdout()),
        Shell::Zsh => generate(Zsh, &mut cmd, "leetrs", &mut io::stdout()),
        Shell::Fish => generate(Fish, &mut cmd, "leetrs", &mut io::stdout()),
        Shell::Elvish => todo!(),
        Shell::PowerShell => todo!(),
        _ => todo!(),
    }
}

/// Fetches the full problem list and user data, then launches the interactive TUI.
pub async fn open_tui(language: &Option<Language>) {
    let creds = match LeetCodeCredentials::load() {
        Some(c) => c,
        None => {
            eprintln!("❌ Not authenticated. Please run `leetrs auth` first.");
            return;
        }
    };
    let client = match LeetCodeClient::new(creds) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("❌ Failed to initialize client: {}", e);
            return;
        }
    };
    let picker = Picker::new(client);

    let problems = match picker.list_problems().await {
        Ok(p) => p,
        Err(e) => {
            eprintln!("❌ Failed to fetch problems: {}", e);
            return;
        }
    };

    let problems: Rc<[ProblemSummary]> = Rc::from(problems);
    let user_data = picker.get_user_data().await.ok();
    if let Err(e) = crate::tui::run_tui(Rc::clone(&problems), picker, user_data, language).await {
        eprintln!("❌ TUI error: {}", e);
    }
}

/// Resolves and writes the problem files, then opens the configured editor.
///
/// When `preview` is true, the Markdown description is printed to stdout
/// instead of opening an editor.
pub async fn pick_and_open(
    picker: &Picker,
    identifier: &Identifier,
    language: &Option<Language>,
    preview: bool,
) -> std::result::Result<(), String> {
    let (code, desc) = picker
        .pick(identifier, language)
        .await
        .map_err(|e| format!("{}", e))?;

    if !preview {
        let config = CONFIG
            .get()
            .ok_or_else(|| "Failed to initialise config".to_string())?;
        let editor = config.editor.as_deref().unwrap_or("nvim");
        let show_description = config.show_description.unwrap_or(true);

        println!("🚀 launching {}...", editor);

        let status = if show_description {
            if editor.contains("nvim") || editor.contains("vim") {
                Command::new(editor)
                    .arg(&desc)
                    .arg("-c")
                    .arg(format!("vsplit {}", code))
                    .status()
            } else {
                Command::new(editor).arg(&desc).arg(&code).status()
            }
        } else {
            Command::new(editor).arg(&code).status()
        };

        match status {
            Ok(exit_status) if exit_status.success() => {
                println!("\n👋 {} closed.", editor);
            }
            Ok(exit_status) => {
                eprintln!("⚠️ {} exited with an error code: {}", editor, exit_status);
            }
            Err(e) => {
                eprintln!(
                    "❌ failed to launch {}. is it installed and in your path? error: {}",
                    editor, e
                );
            }
        }
    } else {
        let content = fs::read_to_string(desc)
            .map_err(|e| format!("Failed to read description file: {}", e))?;
        print!("{}", content);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn premium_only_gate_by_premium_status() {
        assert_eq!(premium_only_gate(Some(true)), Ok(None));
        assert!(premium_only_gate(Some(false)).is_err());
        assert!(matches!(premium_only_gate(None), Ok(Some(_))));
    }

    #[test]
    fn test_resolve_custom_cases_none_uses_default() {
        assert_eq!(resolve_custom_cases(None, None), Ok(None));
    }

    #[test]
    fn test_resolve_custom_cases_inline_unescapes_newlines() {
        assert_eq!(
            resolve_custom_cases(Some("[3,2,4]\\n6"), None),
            Ok(Some("[3,2,4]\n6".to_string()))
        );
    }

    #[test]
    fn test_resolve_custom_cases_reads_file() {
        let path = std::env::temp_dir().join(format!("leetrs-cases-{}.txt", std::process::id()));
        fs::write(&path, "[2,7,11,15]\n9\n").unwrap();
        let result = resolve_custom_cases(None, Some(&path));
        fs::remove_file(&path).unwrap();
        assert_eq!(result, Ok(Some("[2,7,11,15]\n9".to_string())));
    }

    #[test]
    fn test_resolve_custom_cases_missing_file_errors() {
        let path = Path::new("definitely_missing_cases_file_xyz.txt");
        let err = resolve_custom_cases(None, Some(path)).unwrap_err();
        assert!(err.contains("Failed to read cases file"));
        assert!(err.contains("definitely_missing_cases_file_xyz.txt"));
    }

    #[test]
    fn test_resolve_custom_cases_keeps_escapes_inside_strings() {
        assert_eq!(
            resolve_custom_cases(Some(r#""dir\n\tfile.ext"\n2"#), None),
            Ok(Some("\"dir\\n\\tfile.ext\"\n2".to_string()))
        );
        assert_eq!(
            resolve_custom_cases(Some(r#""a\\"\n"b""#), None),
            Ok(Some("\"a\\\\\"\n\"b\"".to_string()))
        );
    }

    #[test]
    fn test_resolve_custom_cases_rejects_empty_input() {
        assert!(resolve_custom_cases(Some(""), None).is_err());
        assert!(resolve_custom_cases(Some("  "), None).is_err());
    }

    #[test]
    fn test_resolve_custom_cases_normalizes_crlf_and_rejects_blank_file() {
        let path = std::env::temp_dir().join(format!("leetrs-crlf-{}.txt", std::process::id()));
        fs::write(&path, "[1,2]\r\n3\r\n").unwrap();
        let crlf = resolve_custom_cases(None, Some(&path));
        fs::write(&path, "\n\n").unwrap();
        let blank = resolve_custom_cases(None, Some(&path));
        fs::remove_file(&path).unwrap();
        assert_eq!(crlf, Ok(Some("[1,2]\n3".to_string())));
        assert!(blank.is_err());
    }

    #[test]
    fn test_resolve_custom_cases_inline_trims_trailing_newlines_and_crlf() {
        assert_eq!(
            resolve_custom_cases(Some("[3,2,4]\\n6\\n\\n"), None),
            Ok(Some("[3,2,4]\n6".to_string()))
        );
        assert_eq!(
            resolve_custom_cases(Some("[3,2,4]\r\n6\r\n"), None),
            Ok(Some("[3,2,4]\n6".to_string()))
        );
    }

    #[test]
    fn test_resolve_custom_cases_rejects_both_sources() {
        let err = resolve_custom_cases(Some("1"), Some(Path::new("x.txt"))).unwrap_err();
        assert!(err.contains("cannot specify both"));
    }
}
