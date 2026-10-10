//! macOS Keychain key extraction and AES-128-CBC cookie decryption for Chromium.

use crate::{cookie::Browser, errors::ExtractError};
use aes::Aes128;
use cbc::cipher::{BlockDecryptMut, KeyIvInit};
use pbkdf2::pbkdf2_hmac;
use sha1::Sha1;
use std::process::Command;

type Aes128CbcDec = cbc::Decryptor<Aes128>;

/// Retrieves the browser's Safe Storage password from the macOS Keychain.
pub fn get_macos_key(browser: Browser) -> Result<Vec<u8>, ExtractError> {
    let service_name = match browser {
        Browser::Brave => "Brave Safe Storage",
        Browser::Edge => "Microsoft Edge Safe Storage",
        Browser::Arc => "Arc Safe Storage",
        _ => "Chrome Safe Storage",
    };

    let account_name = match browser {
        Browser::Brave => "Brave",
        Browser::Edge => "Microsoft Edge",
        Browser::Arc => "Arc",
        _ => "Chrome",
    };

    log::debug!(
        "Querying macOS Keychain for service: '{}', account: '{}'",
        service_name,
        account_name
    );

    // Try reading via macOS `security` CLI
    let output = Command::new("security")
        .args([
            "find-generic-password",
            "-w",
            "-s",
            service_name,
            "-a",
            account_name,
        ])
        .output();

    let password = match output {
        Ok(out) if out.status.success() => {
            log::debug!("Successfully retrieved Keychain password using service + account query");
            String::from_utf8_lossy(&out.stdout).trim().to_string()
        }
        _ => {
            log::debug!(
                "Exact service+account query failed; trying fallback query with service only ('{}')...",
                service_name
            );
            // Fallback: try querying just by service name without account
            let fallback_out = Command::new("security")
                .args(["find-generic-password", "-w", "-s", service_name])
                .output()
                .map_err(|e| ExtractError::KeyRetrievalError(e.to_string()))?;

            if !fallback_out.status.success() {
                log::debug!("macOS Keychain fallback query also failed");
                return Err(ExtractError::KeyRetrievalError(format!(
                    "Failed to query Keychain for service '{}'",
                    service_name
                )));
            }
            log::debug!("Successfully retrieved Keychain password using fallback service query");
            String::from_utf8_lossy(&fallback_out.stdout)
                .trim()
                .to_string()
        }
    };

    log::debug!("Deriving 128-bit key via PBKDF2 (1003 iterations, salt: 'saltysalt')...");
    let mut key = [0u8; 16];
    pbkdf2_hmac::<Sha1>(password.as_bytes(), b"saltysalt", 1003, &mut key);
    Ok(key.to_vec())
}

/// Decrypts a `v10` prefixed encrypted cookie value on macOS.
pub fn decrypt_v10_macos(key: &[u8], encrypted_value: &[u8]) -> Result<String, ExtractError> {
    if encrypted_value.len() < 3 {
        return Ok(String::from_utf8_lossy(encrypted_value).to_string());
    }

    if &encrypted_value[..3] != b"v10" && &encrypted_value[..3] != b"v11" {
        return Ok(String::from_utf8_lossy(encrypted_value).to_string());
    }

    let iv = [0x20u8; 16]; // 16 ASCII space characters
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
        .map_err(|e| ExtractError::DatabaseError(format!("AES-CBC decryption error: {:?}", e)))?;

    // On macOS, Chromium prepends a 32-byte SHA-256 HMAC/signature to the plaintext before encryption
    let payload = if decrypted.len() >= 32 {
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
        let result = decrypt_v10_macos(&key, plain).expect("Failed to parse plaintext");
        assert_eq!(result, "unencrypted_value");
    }

    #[test]
    fn test_real_macos_chrome_key_and_ciphertext() {
        if let Ok(key) = get_macos_key(Browser::Chrome) {
            let hex_str = "7631306FF11E4D890DB82AF21582B923A29D4315063A6D27126BD8B73E79895F36C7ACEE402C0F39AC35383D0A16B301B172D81891F5457FE5FEC34EF886251CE2F72C";
            let bytes = (0..hex_str.len())
                .step_by(2)
                .map(|i| u8::from_str_radix(&hex_str[i..i + 2], 16).unwrap())
                .collect::<Vec<u8>>();
            let decrypted = decrypt_v10_macos(&key, &bytes);
            println!("Decrypted result: {:?}", decrypted);
            assert!(decrypted.is_ok());
            let val = decrypted.unwrap();
            println!("Decrypted value: {}", val);
            assert!(!val.is_empty());
        }
    }
}
