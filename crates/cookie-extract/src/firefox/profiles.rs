//! Profile discovery for Mozilla Firefox and forks across Linux distros, macOS, and Windows.

use crate::errors::ExtractError;
use directories::BaseDirs;
use std::fs;
use std::path::PathBuf;

/// Discovers the active default Firefox `cookies.sqlite` database file across
/// standard native packages, Ubuntu Snap, Flatpak, and privacy forks.
pub fn find_firefox_cookie_db() -> Result<PathBuf, ExtractError> {
    let base_dirs = BaseDirs::new()
        .ok_or_else(|| ExtractError::DatabaseError("No home directory found".into()))?;

    let mut candidate_roots: Vec<PathBuf> = Vec::new();

    #[cfg(target_os = "macos")]
    {
        candidate_roots.push(base_dirs.data_dir().join("Firefox"));
        candidate_roots.push(base_dirs.home_dir().join(".mozilla/firefox"));
        candidate_roots.push(base_dirs.data_dir().join("LibreWolf"));
        candidate_roots.push(base_dirs.data_dir().join("Floorp"));
        candidate_roots.push(base_dirs.data_dir().join("Waterfox"));
    }

    #[cfg(target_os = "linux")]
    {
        let home = base_dirs.home_dir();
        let config_dir = base_dirs.config_dir();
        let data_dir = base_dirs.data_dir();

        // 1. Native distro packages (Debian, Ubuntu, Fedora, openSUSE)
        candidate_roots.push(home.join(".mozilla/firefox"));
        candidate_roots.push(home.join(".mozilla"));

        // 2. Arch Linux & XDG standard paths (~/.config/mozilla/firefox, ~/.config/firefox, ~/.local/share)
        candidate_roots.push(config_dir.join("mozilla/firefox"));
        candidate_roots.push(config_dir.join("mozilla"));
        candidate_roots.push(config_dir.join("firefox"));
        candidate_roots.push(data_dir.join("mozilla/firefox"));
        candidate_roots.push(data_dir.join("firefox"));

        // 3. Ubuntu default Snap package
        candidate_roots.push(home.join("snap/firefox/common/.mozilla/firefox"));
        candidate_roots.push(home.join("snap/firefox/current/.mozilla/firefox"));

        // 4. Flatpak sandbox (Fedora Silverblue, Flathub, SteamOS)
        candidate_roots.push(home.join(".var/app/org.mozilla.firefox/.mozilla/firefox"));
        candidate_roots.push(home.join(".var/app/org.mozilla.firefox/config/mozilla/firefox"));

        // 5. Common privacy forks on Linux
        candidate_roots.push(home.join(".librewolf"));
        candidate_roots.push(config_dir.join("librewolf"));
        candidate_roots.push(home.join(".var/app/io.gitlab.librewolf-community/.librewolf"));
        candidate_roots.push(home.join(".floorp"));
        candidate_roots.push(config_dir.join("floorp"));
        candidate_roots.push(home.join(".waterfox"));
        candidate_roots.push(config_dir.join("waterfox"));
    }

    #[cfg(target_os = "windows")]
    {
        candidate_roots.push(base_dirs.config_dir().join("Mozilla/Firefox"));
        candidate_roots.push(base_dirs.config_dir().join("LibreWolf"));
        candidate_roots.push(base_dirs.config_dir().join("Waterfox"));
    }

    // Try finding via profiles.ini across all candidate roots
    for profiles_root in &candidate_roots {
        let ini_path = profiles_root.join("profiles.ini");
        if let Ok(ini_content) = fs::read_to_string(&ini_path)
            && let Some((profile_subpath, is_relative)) = parse_default_profile_info(&ini_content)
        {
            let profile_dir = if is_relative {
                profiles_root.join(profile_subpath)
            } else {
                PathBuf::from(profile_subpath)
            };

            let cookie_db = profile_dir.join("cookies.sqlite");
            if cookie_db.exists() {
                return Ok(cookie_db);
            }
        }

        // Direct search fallback in case profiles.ini is absent or custom-named
        if let Ok(entries) = fs::read_dir(profiles_root) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let direct_cookie_db = path.join("cookies.sqlite");
                    if direct_cookie_db.exists() {
                        return Ok(direct_cookie_db);
                    }
                }
            }
        }
    }

    Err(ExtractError::DatabaseNotFound(
        candidate_roots
            .into_iter()
            .next()
            .unwrap_or_else(|| base_dirs.home_dir().join(".mozilla/firefox")),
    ))
}

/// Parses the default profile path and `is_relative` flag from a `profiles.ini` string.
pub fn parse_default_profile_info(ini_content: &str) -> Option<(String, bool)> {
    let mut current_path = None;
    let mut current_is_relative = true;
    let mut is_default = false;
    let mut first_profile = None;

    for line in ini_content.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            if is_default && current_path.is_some() {
                return current_path.map(|p| (p, current_is_relative));
            }
            current_path = None;
            current_is_relative = true;
            is_default = false;
        } else if let Some(val) = line.strip_prefix("Path=") {
            let path_str = val.trim().to_string();
            if first_profile.is_none() {
                first_profile = Some((path_str.clone(), current_is_relative));
            }
            current_path = Some(path_str);
        } else if let Some(val) = line.strip_prefix("IsRelative=") {
            current_is_relative = val.trim() != "0";
        } else if line == "Default=1" || line.starts_with("Default=") {
            is_default = true;
        }
    }

    if is_default && current_path.is_some() {
        current_path.map(|p| (p, current_is_relative))
    } else {
        first_profile
    }
}

/// Backwards-compatible helper returning relative or absolute path string.
pub fn parse_default_profile_path(ini_content: &str) -> Option<String> {
    parse_default_profile_info(ini_content).map(|(path, _)| path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_default_profile_from_ini_content() {
        let ini = r#"
[Profile0]
Name=default-release
IsRelative=1
Path=Profiles/abc12345.default-release
Default=1

[General]
StartWithLastProfile=1
"#;
        let (path, is_rel) =
            parse_default_profile_info(ini).expect("Failed to parse default profile");
        assert_eq!(path, "Profiles/abc12345.default-release");
        assert!(is_rel);
    }

    #[test]
    fn parses_absolute_profile_path() {
        let ini = r#"
[Profile0]
Name=custom
IsRelative=0
Path=/custom/firefox/profile
Default=1
"#;
        let (path, is_rel) = parse_default_profile_info(ini).expect("Failed to parse profile");
        assert_eq!(path, "/custom/firefox/profile");
        assert!(!is_rel);
    }

    #[test]
    fn falls_back_to_first_profile_if_no_default_flag() {
        let ini = r#"
[Profile0]
Name=default
IsRelative=1
Path=Profiles/xyz67890.default
"#;
        let (path, is_rel) =
            parse_default_profile_info(ini).expect("Failed to parse first profile");
        assert_eq!(path, "Profiles/xyz67890.default");
        assert!(is_rel);
    }
}
