//! Profile discovery for Mozilla Firefox across macOS, Linux, and Windows.

use crate::errors::ExtractError;
use directories::BaseDirs;
use std::fs;
use std::path::PathBuf;

/// Discovers the active default Firefox `cookies.sqlite` database file.
pub fn find_firefox_cookie_db() -> Result<PathBuf, ExtractError> {
    let base_dirs = BaseDirs::new()
        .ok_or_else(|| ExtractError::DatabaseError("No home directory found".into()))?;

    #[cfg(target_os = "macos")]
    let profiles_root = base_dirs.data_dir().join("Firefox");

    #[cfg(target_os = "linux")]
    let profiles_root = base_dirs.home_dir().join(".mozilla/firefox");

    #[cfg(target_os = "windows")]
    let profiles_root = base_dirs.config_dir().join("Mozilla/Firefox");

    let ini_path = profiles_root.join("profiles.ini");
    if !ini_path.exists() {
        return Err(ExtractError::DatabaseNotFound(ini_path));
    }

    let ini_content = fs::read_to_string(&ini_path)
        .map_err(|e| ExtractError::DatabaseError(format!("Cannot read profiles.ini: {}", e)))?;

    let relative_path = parse_default_profile_path(&ini_content).ok_or_else(|| {
        ExtractError::DatabaseError("No default profile found in profiles.ini".into())
    })?;

    let cookie_db = profiles_root.join(relative_path).join("cookies.sqlite");
    if cookie_db.exists() {
        Ok(cookie_db)
    } else {
        Err(ExtractError::DatabaseNotFound(cookie_db))
    }
}

/// Parses the default profile path from a `profiles.ini` file string.
pub fn parse_default_profile_path(ini_content: &str) -> Option<String> {
    let mut current_path = None;
    let mut is_default = false;
    let mut first_path = None;

    for line in ini_content.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            if is_default && current_path.is_some() {
                return current_path;
            }
            current_path = None;
            is_default = false;
        } else if let Some(val) = line.strip_prefix("Path=") {
            let path_str = val.trim().to_string();
            if first_path.is_none() {
                first_path = Some(path_str.clone());
            }
            current_path = Some(path_str);
        } else if line == "Default=1" || line.starts_with("Default=") {
            is_default = true;
        }
    }

    if is_default && current_path.is_some() {
        current_path
    } else {
        first_path
    }
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
        let path = parse_default_profile_path(ini).expect("Failed to parse default profile");
        assert_eq!(path, "Profiles/abc12345.default-release");
    }

    #[test]
    fn falls_back_to_first_profile_if_no_default_flag() {
        let ini = r#"
[Profile0]
Name=default
IsRelative=1
Path=Profiles/xyz67890.default
"#;
        let path = parse_default_profile_path(ini).expect("Failed to parse first profile");
        assert_eq!(path, "Profiles/xyz67890.default");
    }
}
