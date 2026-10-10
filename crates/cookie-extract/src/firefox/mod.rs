//! Native Mozilla Firefox cookie extractor.

pub mod profiles;

use crate::{
    cookie::Cookie, errors::ExtractError, sqlite::SafeSqliteReader, traits::CookieExtractor,
};

/// Cookie extractor for Firefox profiles (plaintext `moz_cookies` table).
pub struct FirefoxExtractor;

impl CookieExtractor for FirefoxExtractor {
    fn extract_cookies(&self, domains: &[&str]) -> Result<Vec<Cookie>, ExtractError> {
        let db_path = profiles::find_firefox_cookie_db()?;
        log::debug!("Opening Firefox SQLite reader at {:?}", db_path);
        let reader = SafeSqliteReader::open_copy_at(&db_path)?;

        let mut all_cookies = Vec::new();
        for domain in domains {
            let sql = format!(
                "SELECT name, value, host, path, expiry FROM moz_cookies WHERE host LIKE '%{}%'",
                domain
            );
            let cookies = reader.query(&sql, |row| {
                Ok(Cookie {
                    name: row.get(0)?,
                    value: row.get(1)?,
                    domain: row.get(2)?,
                    path: row.get(3)?,
                    expires_at: row.get(4).ok(),
                })
            })?;
            log::debug!(
                "Found {} Firefox cookies matching domain filter '%{}%'",
                cookies.len(),
                domain
            );
            all_cookies.extend(cookies);
        }

        Ok(all_cookies)
    }
}
