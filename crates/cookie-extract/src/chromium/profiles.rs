//! Chromium profile discovery for Chrome, Brave, Edge, Chromium, and Arc.

use crate::{cookie::Browser, errors::ExtractError};
use directories::BaseDirs;
use std::path::PathBuf;

/// Locates the `Cookies` or `Network/Cookies` database for a given Chromium browser.
pub fn find_chromium_cookie_db(browser: Browser) -> Result<PathBuf, ExtractError> {
    let base_dirs = BaseDirs::new()
        .ok_or_else(|| ExtractError::DatabaseError("No home directory found".into()))?;

    let browser_dir_name = match browser {
        Browser::Chrome => "Google/Chrome",
        Browser::Brave => "BraveSoftware/Brave-Browser",
        Browser::Edge => "Microsoft Edge",
        Browser::Arc => "Arc/User Data",
        _ => "Google/Chrome",
    };

    #[cfg(target_os = "macos")]
    let user_data_dir = base_dirs.data_dir().join(browser_dir_name);

    #[cfg(target_os = "linux")]
    let user_data_dir = {
        let linux_dir = match browser {
            Browser::Chrome => "google-chrome",
            Browser::Brave => "BraveSoftware/Brave-Browser",
            Browser::Edge => "microsoft-edge",
            _ => "google-chrome",
        };
        base_dirs.config_dir().join(linux_dir)
    };

    #[cfg(target_os = "windows")]
    let user_data_dir = {
        let win_dir = match browser {
            Browser::Chrome => "Google/Chrome/User Data",
            Browser::Brave => "BraveSoftware/Brave-Browser/User Data",
            Browser::Edge => "Microsoft/Edge/User Data",
            _ => "Google/Chrome/User Data",
        };
        base_dirs.config_dir().join(win_dir)
    };

    // Candidate profile directories in order
    let profile_candidates = ["Default", "Profile 1", "Profile 2", ""];

    for profile in profile_candidates {
        let profile_dir = if profile.is_empty() {
            user_data_dir.clone()
        } else {
            user_data_dir.join(profile)
        };

        // Modern Chromium stores in Network/Cookies, legacy stores in Cookies
        let network_cookies = profile_dir.join("Network").join("Cookies");
        if network_cookies.exists() {
            return Ok(network_cookies);
        }

        let legacy_cookies = profile_dir.join("Cookies");
        if legacy_cookies.exists() {
            return Ok(legacy_cookies);
        }
    }

    Err(ExtractError::DatabaseNotFound(user_data_dir))
}
