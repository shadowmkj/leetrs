use std::path::PathBuf;

pub mod error;

pub mod auth;

pub mod models;

pub mod client;

pub mod picker;

pub mod tui;

pub mod config;

pub fn get_config_file() -> PathBuf {
    let path = get_config_path().join("config.toml");
    if let Err(e) = std::fs::create_dir_all(get_config_path()) {
        eprintln!("Failed to create config directory: {}", e);
    }
    path
}

pub fn get_config_path() -> PathBuf {
    let path = directories::BaseDirs::new()
        .expect("Failed to find directories")
        .home_dir()
        .join(".config/leetrs");
    if !path.exists() {
        if let Err(e) = std::fs::create_dir_all(&path) {
            eprintln!("Failed to create config directory: {}", e);
        }
    }
    path
}
