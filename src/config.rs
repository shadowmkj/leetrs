use anyhow::{Context, Result};
use std::{fs, sync::OnceLock};

use serde::Deserialize;

use crate::get_config_file;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub editor: Option<String>,
    pub language: Option<String>,
    pub show_description: Option<bool>,
}

impl Config {
    pub fn new() -> Result<Self> {
        let config_path = get_config_file();
        let config_data = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file at {:?}", config_path))?;
        let parsed: Self = toml::from_str(&config_data).context("Failed to parse TOML config")?;
        Ok(parsed)
    }
}

pub static CONFIG: OnceLock<Config> = OnceLock::new();
