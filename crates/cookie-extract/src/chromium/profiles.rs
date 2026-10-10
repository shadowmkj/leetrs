//! Chromium profile discovery for Chrome, Brave, Edge, Chromium, and Arc across OSes.

use crate::{cookie::Browser, errors::ExtractError};
use directories::BaseDirs;
use std::path::PathBuf;

/// Locates the `Cookies` or `Network/Cookies` database for a given Chromium browser.
pub fn find_chromium_cookie_db(browser: Browser) -> Result<PathBuf, ExtractError> {
    let base_dirs = BaseDirs::new()
        .ok_or_else(|| ExtractError::DatabaseError("No home directory found".into()))?;

    let mut candidate_user_data_dirs: Vec<PathBuf> = Vec::new();

    #[cfg(target_os = "macos")]
    {
        let browser_dir_name = match browser {
            Browser::Chrome => "Google/Chrome",
            Browser::Brave => "BraveSoftware/Brave-Browser",
            Browser::Edge => "Microsoft Edge",
            Browser::Arc => "Arc/User Data",
            _ => "Google/Chrome",
        };
        candidate_user_data_dirs.push(base_dirs.data_dir().join(browser_dir_name));
    }

    #[cfg(target_os = "linux")]
    {
        let config_dir = base_dirs.config_dir();
        let home = base_dirs.home_dir();

        match browser {
            Browser::Chrome => {
                candidate_user_data_dirs.push(config_dir.join("google-chrome"));
                candidate_user_data_dirs.push(config_dir.join("google-chrome-beta"));
                candidate_user_data_dirs.push(config_dir.join("google-chrome-unstable"));
                candidate_user_data_dirs.push(config_dir.join("chromium"));
                candidate_user_data_dirs
                    .push(home.join(".var/app/com.google.Chrome/config/google-chrome"));
                candidate_user_data_dirs
                    .push(home.join(".var/app/org.chromium.Chromium/config/chromium"));
                candidate_user_data_dirs.push(home.join("snap/chromium/common/chromium"));
                candidate_user_data_dirs.push(home.join("snap/chromium/current/.config/chromium"));
            }
            Browser::Brave => {
                candidate_user_data_dirs.push(config_dir.join("BraveSoftware/Brave-Browser"));
                candidate_user_data_dirs.push(
                    home.join(".var/app/com.brave.Browser/config/BraveSoftware/Brave-Browser"),
                );
                candidate_user_data_dirs
                    .push(home.join("snap/brave/current/.config/BraveSoftware/Brave-Browser"));
            }
            Browser::Edge => {
                candidate_user_data_dirs.push(config_dir.join("microsoft-edge"));
                candidate_user_data_dirs.push(config_dir.join("microsoft-edge-dev"));
                candidate_user_data_dirs
                    .push(home.join(".var/app/com.microsoft.Edge/config/microsoft-edge"));
            }
            _ => {
                candidate_user_data_dirs.push(config_dir.join("google-chrome"));
                candidate_user_data_dirs.push(config_dir.join("chromium"));
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        let win_dir = match browser {
            Browser::Chrome => "Google/Chrome/User Data",
            Browser::Brave => "BraveSoftware/Brave-Browser/User Data",
            Browser::Edge => "Microsoft/Edge/User Data",
            _ => "Google/Chrome/User Data",
        };
        candidate_user_data_dirs.push(base_dirs.config_dir().join(win_dir));
    }

    let profile_candidates = ["Default", "Profile 1", "Profile 2", ""];

    for user_data_dir in &candidate_user_data_dirs {
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
    }

    Err(ExtractError::DatabaseNotFound(
        candidate_user_data_dirs
            .into_iter()
            .next()
            .unwrap_or_else(|| base_dirs.config_dir().to_path_buf()),
    ))
}
