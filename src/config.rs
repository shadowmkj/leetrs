use anyhow::{Context, Result};
use std::{fs, rc::Rc};

use serde::Deserialize;

use crate::get_config_file;

#[derive(Debug, Deserialize)]
struct Config {
    editor: Option<String>,
}

impl Config {
    pub fn new() -> Result<Rc<Self>> {
        let config_path = get_config_file();
        let config_data = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file at {:?}", config_path))?;
        let parsed: Self = toml::from_str(&config_data).context("Failed to parse TOML config")?;
        Ok(Rc::new(parsed))
    }
}
