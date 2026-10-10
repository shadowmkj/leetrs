//! Authentication helpers for the leetrs CLI.
//!
//! Credentials are persisted as a JSON file in the OS-standard config directory
//! (`ProjectDirs::config_dir()` via the `directories` crate, e.g.
//! `~/.config/leetrs/` on Linux / macOS).
use cookie_extract::{Browser, extract, parse_raw_cookie_header};
use dialoguer::Password;
use dialoguer::theme::ColorfulTheme;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// The two cookies required to authenticate all requests to LeetCode's API.
///
/// Both values are obtained either by extracting them directly from a running
/// browser session ([`auto_extract_flow`]) or by having the user paste them
/// manually ([`manual_auth_flow`]).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct LeetCodeCredentials {
    /// Value of the `LEETCODE_SESSION` cookie.
    pub session_cookie: String,
    /// Value of the `csrftoken` cookie (also sent as the `x-csrftoken` request header).
    pub csrf_token: String,
}

impl LeetCodeCredentials {
    /// Returns the path to `credentials.json` inside the OS config directory.
    fn get_config_path() -> Option<PathBuf> {
        let dirs = ProjectDirs::from("com", "shadowmkj", "leetrs")?;
        Some(dirs.config_dir().join("credentials.json"))
    }

    /// Loads credentials from disk. Returns `None` if the file doesn't exist
    /// or cannot be parsed (e.g. after a format change).
    pub fn load() -> Option<Self> {
        let config_path = Self::get_config_path()?;
        let file_contents = std::fs::read_to_string(config_path).ok()?;
        serde_json::from_str(&file_contents).ok()
    }

    /// Serialises credentials to disk as pretty-printed JSON, creating any
    /// missing parent directories along the way.
    pub fn save(&self) -> Result<(), std::io::Error> {
        let config_path = Self::get_config_path().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not determine config directory",
            )
        })?;

        // Ensure the parent directory exists
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(self)?;
        println!("writing to {:?}", config_path);
        fs::write(config_path, json)?;
        Ok(())
    }
}

/// Handles prompting the user to paste their cURL command, raw `Cookie:` header, or tokens.
pub fn manual_auth_flow() -> Result<LeetCodeCredentials, String> {
    println!("\nPaste your cURL command, 'Cookie:' header, or tokens from your browser session:");
    println!("(Developer Tools -> Network tab -> Right click any request -> 'Copy as cURL')\n");

    let raw_input = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Paste header or cURL command")
        .interact()
        .map_err(|e| e.to_string())?;

    let cookies = parse_raw_cookie_header(&raw_input)
        .map_err(|e| format!("Could not parse cookie string: {}", e))?;

    log::debug!("Manual auth parsed {} total cookies", cookies.len());

    let mut session_cookie = None;
    let mut csrf_token = None;

    for cookie in cookies {
        if cookie.name == "LEETCODE_SESSION" {
            session_cookie = Some(cookie.value);
        } else if cookie.name == "csrftoken" {
            csrf_token = Some(cookie.value);
        }
    }

    log::debug!(
        "Manual auth tokens found: LEETCODE_SESSION={}, csrftoken={}",
        session_cookie.is_some(),
        csrf_token.is_some()
    );

    match (session_cookie, csrf_token) {
        (Some(session), Some(csrf)) => Ok(LeetCodeCredentials {
            session_cookie: session,
            csrf_token: csrf,
        }),
        _ => {
            Err("Could not find both LEETCODE_SESSION and csrftoken in the provided input.".into())
        }
    }
}

/// Automatically extracts LeetCode cookies from the specified browser.
pub fn auto_extract_flow(browser_name: &str) -> Result<LeetCodeCredentials, String> {
    let browser = match browser_name.to_lowercase().as_str() {
        "auto" | "autodetect" => Browser::AutoDetect,
        "chrome" => Browser::Chrome,
        "firefox" => Browser::Firefox,
        "brave" => Browser::Brave,
        "edge" => Browser::Edge,
        "arc" => Browser::Arc,
        _ => return Err("Unsupported browser".into()),
    };

    println!(
        "\n🔍 Attempting to extract cookies from {}...",
        browser.as_str()
    );

    log::debug!("Starting browser cookie extraction for {:?}", browser);

    let domains = ["leetcode.com", ".leetcode.com"];
    let cookies = extract(browser, &domains).map_err(|e| {
        log::debug!("Browser extraction failed with error: {:?}", e);
        format!("{} extraction failed: {}", browser.as_str(), e)
    })?;

    log::debug!(
        "Extracted {} candidate cookies from {:?}",
        cookies.len(),
        browser
    );

    let mut session_cookie = None;
    let mut csrf_token = None;

    for cookie in cookies {
        log::debug!("Cookie: {:?}", cookie);
        if cookie.name == "LEETCODE_SESSION" {
            session_cookie = Some(cookie.value);
        } else if cookie.name == "csrftoken" {
            csrf_token = Some(cookie.value);
        }
    }

    log::debug!(
        "Auto extraction tokens found: LEETCODE_SESSION={}, csrftoken={}",
        session_cookie.is_some(),
        csrf_token.is_some()
    );

    match (session_cookie, csrf_token) {
        (Some(session), Some(csrf)) => Ok(LeetCodeCredentials {
            session_cookie: session,
            csrf_token: csrf,
        }),
        _ => Err(
            "Could not find both LEETCODE_SESSION and csrftoken in the browser's database.".into(),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // LeetCodeCredentials: serde round-trip
    // -----------------------------------------------------------------------

    #[test]
    fn credentials_json_contains_expected_field_names() {
        let creds = LeetCodeCredentials {
            session_cookie: "sess_123".to_string(),
            csrf_token: "csrf_456".to_string(),
        };
        let json = serde_json::to_string(&creds).unwrap();
        assert!(json.contains("session_cookie"));
        assert!(json.contains("csrf_token"));
    }

    #[test]
    fn auto_extract_flow_rejects_unsupported_browsers() {
        for browser in ["safari", "", "opera", "vivaldi"] {
            let result = auto_extract_flow(browser);
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), "Unsupported browser");
        }
    }

    #[test]
    fn auto_extract_flow_recognizes_supported_browser_names() {
        for browser in ["chrome", "firefox", "brave", "edge", "arc", "auto"] {
            // Should not fail with "Unsupported browser"
            let result = auto_extract_flow(browser);
            if let Err(msg) = result {
                assert_ne!(msg, "Unsupported browser");
            }
        }
    }
}
