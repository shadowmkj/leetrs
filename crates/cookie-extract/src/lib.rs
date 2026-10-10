//! `cookie-extract` — Decoupled cross-platform browser cookie extraction and parsing library.

pub mod chromium;
pub mod cookie;
pub mod errors;
pub mod firefox;
pub mod parser;
pub mod sqlite;
pub mod traits;

pub use chromium::ChromiumExtractor;
pub use cookie::{Browser, Cookie};
pub use errors::ExtractError;
pub use firefox::FirefoxExtractor;
pub use parser::parse_raw_cookie_header;
pub use traits::CookieExtractor;

/// Extracts all cookies matching any of the specified domains from the given browser.
pub fn extract(browser: Browser, domains: &[&str]) -> Result<Vec<Cookie>, ExtractError> {
    match browser {
        Browser::Firefox => FirefoxExtractor.extract_cookies(domains),
        Browser::Chrome | Browser::Brave | Browser::Edge | Browser::Arc => {
            ChromiumExtractor { browser }.extract_cookies(domains)
        }
        Browser::AutoDetect => auto_extract(domains),
    }
}

/// Auto-detects installed browsers in order of preference (Firefox -> Chrome -> Brave -> Edge -> Arc)
/// and extracts cookies matching the given domains from the first successful browser found.
pub fn auto_extract(domains: &[&str]) -> Result<Vec<Cookie>, ExtractError> {
    let candidates = [
        Browser::Firefox,
        Browser::Chrome,
        Browser::Brave,
        Browser::Edge,
        Browser::Arc,
    ];

    let mut last_error = None;

    for candidate in candidates {
        match extract(candidate, domains) {
            Ok(cookies) if !cookies.is_empty() => return Ok(cookies),
            Ok(_) => continue,
            Err(e) => {
                last_error = Some(e);
            }
        }
    }

    Err(last_error.unwrap_or(ExtractError::BrowserNotFound(Browser::AutoDetect)))
}
