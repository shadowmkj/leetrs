//! Abstract trait for browser cookie extractors.

use crate::{cookie::Cookie, errors::ExtractError};

/// Common interface for extracting cookies from browser profiles.
pub trait CookieExtractor {
    /// Extracts all cookies matching any of the specified domains.
    fn extract_cookies(&self, domains: &[&str]) -> Result<Vec<Cookie>, ExtractError>;
}
