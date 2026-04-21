//! Authentication helpers for the leetrs CLI.
//!
//! Credentials are persisted as a JSON file in the OS-standard config directory
//! (`ProjectDirs::config_dir()` via the `directories` crate, e.g.
//! `~/.config/leetrs/` on Linux / macOS).
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
#[derive(Serialize, Deserialize, Debug, Clone)]
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
        println!("writing to {:?}", &config_path);
        fs::write(config_path, json)?;
        Ok(())
    }
}

/// Handles prompting the user to paste their tokens manually
pub fn manual_auth_flow() -> Result<LeetCodeCredentials, String> {
    println!("\nPlease extract your cookies from your browser session.");
    println!("(Developer Tools -> Application -> Cookies -> leetcode.com)\n");

    let session_cookie = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter LEETCODE_SESSION cookie")
        .interact()
        .map_err(|e| e.to_string())?;

    let csrf_token = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter csrftoken cookie")
        .interact()
        .map_err(|e| e.to_string())?;

    Ok(LeetCodeCredentials {
        session_cookie,
        csrf_token,
    })
}

/// Automatically extracts LeetCode cookies from the specified browser
pub fn auto_extract_flow(browser: &str) -> Result<LeetCodeCredentials, String> {
    println!("\n🔍 Attempting to extract cookies from {}...", browser);

    // We only want to query cookies belonging to LeetCode to speed up the process
    let domains = Some(vec!["leetcode.com".to_string()]);

    let cookies = match browser {
        "chrome" => {
            rookie::chrome(domains).map_err(|e| format!("Chrome extraction failed: {}", e))?
        }
        "firefox" => {
            rookie::firefox(domains).map_err(|e| format!("Firefox extraction failed: {}", e))?
        }
        _ => return Err("Unsupported browser".into()),
    };

    let mut session_cookie = None;
    let mut csrf_token = None;

    // Search the returned cookies for the two we care about
    for cookie in cookies {
        if cookie.name == "LEETCODE_SESSION" {
            session_cookie = Some(cookie.value);
        } else if cookie.name == "csrftoken" {
            csrf_token = Some(cookie.value);
        }
    }

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
    fn credentials_serde_roundtrip_typical() {
        let creds = LeetCodeCredentials {
            session_cookie: "abc123XYZ".to_string(),
            csrf_token: "tok987".to_string(),
        };
        let json = serde_json::to_string(&creds).expect("serialization must not fail");
        let back: LeetCodeCredentials =
            serde_json::from_str(&json).expect("deserialization must not fail");
        assert_eq!(back.session_cookie, "abc123XYZ");
        assert_eq!(back.csrf_token, "tok987");
    }

    #[test]
    fn credentials_serde_roundtrip_empty_strings() {
        // Tokens can theoretically be empty; the struct must survive that.
        let creds = LeetCodeCredentials {
            session_cookie: "".to_string(),
            csrf_token: "".to_string(),
        };
        let json = serde_json::to_string(&creds).unwrap();
        let back: LeetCodeCredentials = serde_json::from_str(&json).unwrap();
        assert_eq!(back.session_cookie, "");
        assert_eq!(back.csrf_token, "");
    }

    #[test]
    fn credentials_serde_roundtrip_special_characters() {
        // Real LeetCode session cookies contain dots, hyphens, etc.
        let session = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VybmFtZSI6InRlc3QifQ.abc-def";
        let csrf = "Fy3z!@#Kx9";
        let creds = LeetCodeCredentials {
            session_cookie: session.to_string(),
            csrf_token: csrf.to_string(),
        };
        let json = serde_json::to_string(&creds).unwrap();
        let back: LeetCodeCredentials = serde_json::from_str(&json).unwrap();
        assert_eq!(back.session_cookie, session);
        assert_eq!(back.csrf_token, csrf);
    }

    #[test]
    fn credentials_json_contains_expected_field_names() {
        let creds = LeetCodeCredentials {
            session_cookie: "s".to_string(),
            csrf_token: "c".to_string(),
        };
        let json = serde_json::to_string(&creds).unwrap();
        // Field names must match what the file on disk will look like.
        assert!(json.contains("session_cookie"), "missing 'session_cookie' key");
        assert!(json.contains("csrf_token"), "missing 'csrf_token' key");
    }

    #[test]
    fn credentials_clone_is_independent() {
        let original = LeetCodeCredentials {
            session_cookie: "orig_session".to_string(),
            csrf_token: "orig_csrf".to_string(),
        };
        let mut cloned = original.clone();
        cloned.session_cookie = "mutated".to_string();
        // Mutating the clone must not affect the original.
        assert_eq!(original.session_cookie, "orig_session");
        assert_eq!(cloned.session_cookie, "mutated");
    }

    // -----------------------------------------------------------------------
    // auto_extract_flow: reject unsupported browsers without panicking
    // -----------------------------------------------------------------------

    #[test]
    fn auto_extract_flow_rejects_unsupported_browser() {
        let result = auto_extract_flow("safari");
        assert!(result.is_err());
        let msg = result.unwrap_err();
        assert_eq!(msg, "Unsupported browser");
    }

    #[test]
    fn auto_extract_flow_rejects_empty_browser_name() {
        let result = auto_extract_flow("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Unsupported browser");
    }

    #[test]
    fn auto_extract_flow_rejects_case_sensitive_browser_name() {
        // Browser names are matched case-sensitively; "Chrome" != "chrome".
        let result = auto_extract_flow("Chrome");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Unsupported browser");
    }
}
