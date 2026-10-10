//! Native Chromium cookie extractor for Chrome, Brave, Edge, and Arc.

pub mod crypto;
pub mod profiles;

use crate::{
    cookie::{Browser, Cookie},
    errors::ExtractError,
    sqlite::SafeSqliteReader,
    traits::CookieExtractor,
};

/// Cookie extractor for Chromium browser families.
pub struct ChromiumExtractor {
    pub browser: Browser,
}

impl CookieExtractor for ChromiumExtractor {
    fn extract_cookies(&self, domains: &[&str]) -> Result<Vec<Cookie>, ExtractError> {
        let db_path = profiles::find_chromium_cookie_db(self.browser)?;
        log::debug!(
            "Opening Chromium SQLite reader for {:?} at {:?}",
            self.browser,
            db_path
        );
        let reader = SafeSqliteReader::open_copy_at(&db_path)?;

        #[cfg(target_os = "macos")]
        let key = crypto::macos::get_macos_key(self.browser)?;

        #[cfg(target_os = "linux")]
        let keys = crypto::linux::get_linux_keys(self.browser);

        let mut all_cookies = Vec::new();
        for domain in domains {
            let sql = format!(
                "SELECT name, value, encrypted_value, host_key, path, expires_utc FROM cookies WHERE host_key LIKE '%{}%'",
                domain
            );
            let cookies = reader.query(&sql, |row| {
                let name: String = row.get(0)?;
                let plaintext: String = row.get(1)?;
                let encrypted: Vec<u8> = row.get(2)?;
                let domain: String = row.get(3)?;
                let path: String = row.get(4)?;
                let expires: Option<i64> = row.get(5).ok();

                let val = if !plaintext.is_empty() {
                    plaintext
                } else if !encrypted.is_empty() {
                    #[cfg(target_os = "macos")]
                    {
                        crypto::macos::decrypt_v10_macos(&key, &encrypted).unwrap_or_default()
                    }
                    #[cfg(target_os = "linux")]
                    {
                        let mut decrypted = String::new();
                        for key in &keys {
                            if let Ok(res) = crypto::linux::decrypt_v10_linux(key, &encrypted)
                                && !res.is_empty()
                            {
                                decrypted = res;
                                break;
                            }
                        }
                        decrypted
                    }
                    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
                    {
                        String::from_utf8_lossy(&encrypted).to_string()
                    }
                } else {
                    String::new()
                };

                Ok(Cookie {
                    name,
                    value: val,
                    domain,
                    path,
                    expires_at: expires,
                })
            })?;
            log::debug!(
                "Found {} Chromium cookies matching domain filter '%{}%'",
                cookies.len(),
                domain
            );
            all_cookies.extend(cookies);
        }

        Ok(all_cookies)
    }
}
