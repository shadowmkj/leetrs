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
    log::debug!(
        "Extracting cookies for browser {:?} with domain filters: {:?}",
        browser,
        domains
    );
    let result = match browser {
        Browser::Firefox => FirefoxExtractor.extract_cookies(domains),
        Browser::Chrome | Browser::Brave | Browser::Edge | Browser::Arc => {
            ChromiumExtractor { browser }.extract_cookies(domains)
        }
        Browser::AutoDetect => auto_extract(domains),
    };

    match &result {
        Ok(cookies) => log::debug!(
            "Successfully extracted {} cookies for {:?}",
            cookies.len(),
            browser
        ),
        Err(err) => log::debug!("Extraction failed for {:?}: {}", browser, err),
    }

    result
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

    log::debug!(
        "Starting browser auto-detection across candidates: {:?}",
        candidates
    );
    let mut last_error = None;

    for candidate in candidates {
        log::debug!("Attempting auto-detection with browser: {:?}", candidate);
        match extract(candidate, domains) {
            Ok(cookies) if !cookies.is_empty() => {
                log::debug!(
                    "Auto-detection selected {:?} with {} matching cookies",
                    candidate,
                    cookies.len()
                );
                return Ok(cookies);
            }
            Ok(_) => {
                log::debug!(
                    "Browser {:?} succeeded but found 0 matching cookies",
                    candidate
                );
                continue;
            }
            Err(e) => {
                log::debug!("Browser {:?} auto-detection skipped: {}", candidate, e);
                last_error = Some(e);
            }
        }
    }

    Err(last_error.unwrap_or(ExtractError::BrowserNotFound(Browser::AutoDetect)))
}
