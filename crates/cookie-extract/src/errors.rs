//! Strongly typed error definitions for cookie extraction and parsing.

use crate::cookie::Browser;
use std::path::PathBuf;

/// Error conditions that can arise during cookie extraction or parsing.
#[derive(thiserror::Error, Debug)]
pub enum ExtractError {
    #[error("Browser '{0:?}' was not detected on this system")]
    BrowserNotFound(Browser),

    #[error("Cookie database not found at path: {0}")]
    DatabaseNotFound(PathBuf),

    #[error("Failed to read SQLite cookie database: {0}")]
    DatabaseError(String),

    #[error("Failed to retrieve master decryption key: {0}")]
    KeyRetrievalError(String),

    #[error("App-Bound encryption restricted (Windows Chrome 127+); manual paste required")]
    AppBoundRestricted,

    #[error("Failed to parse raw cookie or cURL string")]
    InvalidHeaderFormat,

    #[error("I/O error during cookie extraction: {0}")]
    IoError(#[from] std::io::Error),
}
