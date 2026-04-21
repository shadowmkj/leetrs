//! # leetrs
//!
//! Core library for the `leetrs` CLI — a terminal-first LeetCode client.
//!
//! The crate is split into focused modules:
//! - [`auth`] — credential storage and browser cookie extraction
//! - [`client`] — authenticated HTTP/GraphQL client for the LeetCode API
//! - [`models`] — request/response data types
//! - [`picker`] — problem fetching, file generation, submission, and local caching
//! - [`tui`] — ratatui-based interactive problem browser
//! - [`config`] — TOML user config
use std::path::PathBuf;

pub mod error;

pub mod auth;

pub mod models;

pub mod client;

pub mod picker;

pub mod tui;

pub mod config;

/// Returns the path to `~/.config/leetrs/config.toml`, creating the directory if needed.
pub fn get_config_file() -> PathBuf {
    let path = get_config_path().join("config.toml");
    if let Err(e) = std::fs::create_dir_all(get_config_path()) {
        eprintln!("Failed to create config directory: {}", e);
    }
    path
}

/// Returns `~/.config/leetrs/`, creating it on first use.
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
