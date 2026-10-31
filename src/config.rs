use anyhow::{Context, Result};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::OnceLock,
};

use serde::{Deserialize, Serialize};

use crate::get_config_file;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub editor: Option<String>,
    pub language: Option<String>,
    pub show_description: Option<bool>,
    /// Where `pick` writes problem files, and where `test` and `submit` look
    /// for a file that is not in the current directory. Absolute, `~`-prefixed
    /// and relative paths are accepted (relative ones resolve against the
    /// current directory); unset means the current directory.
    pub solutions_dir: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            editor: Some("nvim".to_string()),
            language: Some("python3".to_string()),
            show_description: Some(true),
            solutions_dir: None,
        }
    }
}

impl Config {
    pub fn new() -> Result<Self> {
        let config_path = get_config_file();
        if !config_path.exists() {
            if let Some(parent) = config_path.parent() {
                fs::create_dir_all(parent).with_context(|| {
                    format!("Failed to create config directory at {:?}", parent)
                })?;
            }

            let default_config = Self::default();
            let default_toml = toml::to_string_pretty(&default_config)
                .context("Failed to serialize default config to TOML")?;

            fs::write(&config_path, default_toml).with_context(|| {
                format!("Failed to create default config file at {:?}", config_path)
            })?;
        }
        let config_data = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file at {:?}", config_path))?;
        let parsed: Self = toml::from_str(&config_data).context("Failed to parse TOML config")?;
        Ok(parsed)
    }
}

pub static CONFIG: OnceLock<Config> = OnceLock::new();

/// The configured `solutions_dir` with `~` expanded. Unset (or config not
/// loaded) gives an empty path, so joined file names stay bare and resolve in
/// the current directory as before.
pub fn solutions_dir() -> PathBuf {
    CONFIG
        .get()
        .and_then(|config| config.solutions_dir.as_deref())
        .map_or_else(PathBuf::new, expand_tilde)
}

/// Expands a leading `~` to the home directory; any other path is returned
/// unchanged.
fn expand_tilde(path: &Path) -> PathBuf {
    match path.strip_prefix("~") {
        Ok(rest) => directories::BaseDirs::new()
            .map(|dirs| dirs.home_dir().join(rest))
            .unwrap_or_else(|| path.to_path_buf()),
        Err(_) => path.to_path_buf(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expand_tilde_uses_the_home_directory() {
        let home = directories::BaseDirs::new()
            .unwrap()
            .home_dir()
            .to_path_buf();
        assert_eq!(expand_tilde(Path::new("~")), home);
        assert_eq!(
            expand_tilde(Path::new("~/Documents/LeetCode")),
            home.join("Documents/LeetCode")
        );
    }

    #[test]
    fn expand_tilde_leaves_other_paths_alone() {
        assert_eq!(expand_tilde(Path::new("/tmp/lc")), PathBuf::from("/tmp/lc"));
        assert_eq!(
            expand_tilde(Path::new("solutions")),
            PathBuf::from("solutions")
        );
        assert_eq!(
            expand_tilde(Path::new("~user/lc")),
            PathBuf::from("~user/lc")
        );
    }

    #[test]
    fn solutions_dir_is_optional_in_toml() {
        let config: Config = toml::from_str("editor = \"nvim\"").unwrap();
        assert_eq!(config.solutions_dir, None);
        let config: Config = toml::from_str("solutions_dir = \"~/lc\"").unwrap();
        assert_eq!(config.solutions_dir, Some(PathBuf::from("~/lc")));
    }
}
