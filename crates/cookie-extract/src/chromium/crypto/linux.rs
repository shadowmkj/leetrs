//! Linux SecretService and fallback password decryption for Chromium.

use crate::{cookie::Browser, errors::ExtractError};
use aes::Aes128;
use cbc::cipher::{BlockDecryptMut, KeyIvInit};
use pbkdf2::pbkdf2_hmac;
use sha1::Sha1;
use std::collections::HashSet;
use std::process::Command;

type Aes128CbcDec = cbc::Decryptor<Aes128>;

/// Retrieves all potential Chromium decryption keys on Linux by querying SecretService
/// via `secret-tool`, KWallet, and the default "peanuts" fallback.
pub fn get_linux_keys(browser: Browser) -> Vec<Vec<u8>> {
    let mut passwords: HashSet<String> = HashSet::new();

    // 1. Build list of secret-tool attribute queries based on browser and common Linux conventions
    let mut secret_tool_queries: Vec<(&str, &str)> = Vec::new();

    match browser {
        Browser::Chrome => {
            secret_tool_queries.push(("application", "chrome"));
            secret_tool_queries.push(("application", "google-chrome"));
            secret_tool_queries.push(("application", "google-chrome-stable"));
            secret_tool_queries.push(("application", "chromium"));
            secret_tool_queries.push(("service", "Chrome Safe Storage"));
            secret_tool_queries.push(("service", "Chromium Safe Storage"));
        }
        Browser::Brave => {
            secret_tool_queries.push(("application", "brave"));
            secret_tool_queries.push(("application", "brave-browser"));
            secret_tool_queries.push(("application", "chromium"));
            secret_tool_queries.push(("service", "Brave Safe Storage"));
            secret_tool_queries.push(("service", "Chromium Safe Storage"));
        }
        Browser::Edge => {
            secret_tool_queries.push(("application", "microsoft-edge"));
            secret_tool_queries.push(("application", "microsoft-edge-dev"));
            secret_tool_queries.push(("service", "Microsoft Edge Safe Storage"));
        }
        _ => {
            secret_tool_queries.push(("application", "chromium"));
            secret_tool_queries.push(("application", "chrome"));
            secret_tool_queries.push(("service", "Chromium Safe Storage"));
            secret_tool_queries.push(("service", "Chrome Safe Storage"));
        }
    }

    // Always check general fallback attributes
    secret_tool_queries.push(("application", "chromium"));
    secret_tool_queries.push(("application", "chrome"));

    log::debug!(
        "Querying SecretService via secret-tool across {} attribute combinations",
        secret_tool_queries.len()
    );

    for (attr, val) in secret_tool_queries {
        if let Ok(out) = Command::new("secret-tool")
            .args(["lookup", attr, val])
            .output()
            && out.status.success()
            && !out.stdout.is_empty()
        {
            let pass = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !pass.is_empty() {
                log::debug!(
                    "Found SecretService password for attribute {}={}",
                    attr,
                    val
                );
                passwords.insert(pass);
            }
        }
    }

    // 2. Try querying KWallet if running KDE Plasma
    let kwallet_queries = [
        ("Chrome Keys", "Chrome Safe Storage"),
        ("Chromium Keys", "Chromium Safe Storage"),
        ("Brave Keys", "Brave Safe Storage"),
    ];

    for (folder, key_name) in kwallet_queries {
        if let Ok(out) = Command::new("kwallet-query")
            .args(["-f", folder, "kdewallet", "-r", key_name])
            .output()
            && out.status.success()
            && !out.stdout.is_empty()
        {
            let pass = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !pass.is_empty() {
                log::debug!(
                    "Found KWallet password in folder {} for key {}",
                    folder,
                    key_name
                );
                passwords.insert(pass);
            }
        }
    }

    // 3. Always include default "peanuts" password
    passwords.insert("peanuts".to_string());

    log::debug!(
        "Deriving Linux AES keys via PBKDF2 for {} distinct password candidates",
        passwords.len()
    );

    // Derive 128-bit keys via PBKDF2 (1 iteration, salt "saltysalt")
    passwords
        .into_iter()
        .map(|password| {
            let mut key = [0u8; 16];
            pbkdf2_hmac::<Sha1>(password.as_bytes(), b"saltysalt", 1, &mut key);
            key.to_vec()
        })
        .collect()
}

/// Decrypts a `v10`/`v11` encrypted cookie value on Linux with a candidate key.
pub fn decrypt_v10_linux(key: &[u8], encrypted_value: &[u8]) -> Result<String, ExtractError> {
    if encrypted_value.len() < 3 {
        return Ok(String::from_utf8_lossy(encrypted_value).to_string());
    }

    if &encrypted_value[..3] != b"v10" && &encrypted_value[..3] != b"v11" {
        return Ok(String::from_utf8_lossy(encrypted_value).to_string());
    }

    let iv = [0x20u8; 16]; // 16 spaces
    let ciphertext = &encrypted_value[3..];
    if ciphertext.is_empty() {
        return Ok(String::new());
    }

    if key.len() < 16 {
        return Err(ExtractError::KeyRetrievalError(
            "Derived key too short".into(),
        ));
    }

    let decryptor = Aes128CbcDec::new((&key[..16]).into(), (&iv).into());
    let mut buf = ciphertext.to_vec();

    let decrypted = decryptor
        .decrypt_padded_mut::<cbc::cipher::block_padding::Pkcs7>(&mut buf)
        .map_err(|e| {
            ExtractError::DatabaseError(format!("Linux AES-CBC decryption error: {:?}", e))
        })?;

    // Check if leading 32 bytes contain non-ASCII binary control bytes (signature/hash header)
    let payload = if decrypted.len() >= 32
        && decrypted[..32]
            .iter()
            .any(|&b| b < 0x20 && b != b'\t' && b != b'\n' && b != b'\r')
    {
        &decrypted[32..]
    } else {
        decrypted
    };

    Ok(String::from_utf8_lossy(payload).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decrypt_plain_unencrypted_value() {
        let key = [0u8; 16];
        let plain = b"unencrypted_value";
        let result = decrypt_v10_linux(&key, plain).expect("Failed to parse plaintext");
        assert_eq!(result, "unencrypted_value");
    }

    #[test]
    fn get_linux_keys_always_includes_at_least_one_key() {
        let keys = get_linux_keys(Browser::Chrome);
        assert!(!keys.is_empty());
    }
}
