//! Linux SecretService and fallback password decryption for Chromium.

use crate::{cookie::Browser, errors::ExtractError};
use aes::Aes128;
use cbc::cipher::{BlockDecryptMut, KeyIvInit};
use pbkdf2::pbkdf2_hmac;
use sha1::Sha1;
use std::process::Command;

type Aes128CbcDec = cbc::Decryptor<Aes128>;

/// Retrieves the Chromium encryption key on Linux using SecretService / `secret-tool`,
/// with a fallback to the default "peanuts" password if no keyring service is running.
pub fn get_linux_key(browser: Browser) -> Result<Vec<u8>, ExtractError> {
    let app_name = match browser {
        Browser::Chrome => "chrome",
        Browser::Brave => "brave",
        Browser::Edge => "microsoft-edge",
        _ => "chromium",
    };

    let service_name = match browser {
        Browser::Chrome => "Chrome Safe Storage",
        Browser::Brave => "Brave Safe Storage",
        Browser::Edge => "Microsoft Edge Safe Storage",
        _ => "Chromium Safe Storage",
    };

    // 1. Try querying secret-tool by application attribute (GNOME Keyring / KWallet standard)
    let password = if let Ok(out) = Command::new("secret-tool")
        .args(["lookup", "application", app_name])
        .output()
    {
        if out.status.success() && !out.stdout.is_empty() {
            String::from_utf8_lossy(&out.stdout).trim().to_string()
        } else if let Ok(out2) = Command::new("secret-tool")
            .args(["lookup", "service", service_name])
            .output()
        {
            if out2.status.success() && !out2.stdout.is_empty() {
                String::from_utf8_lossy(&out2.stdout).trim().to_string()
            } else {
                "peanuts".to_string()
            }
        } else {
            "peanuts".to_string()
        }
    } else {
        "peanuts".to_string()
    };

    let mut key = [0u8; 16];
    pbkdf2_hmac::<Sha1>(password.as_bytes(), b"saltysalt", 1, &mut key);
    Ok(key.to_vec())
}

/// Decrypts a `v10`/`v11` encrypted cookie value on Linux.
pub fn decrypt_v10_linux(key: &[u8], encrypted_value: &[u8]) -> Result<String, ExtractError> {
    if encrypted_value.len() < 3
        || (&encrypted_value[..3] != b"v10" && &encrypted_value[..3] != b"v11")
    {
        return Ok(String::from_utf8_lossy(encrypted_value).to_string());
    }

    let iv = [0x20u8; 16]; // 16 spaces
    let ciphertext = &encrypted_value[3..];
    if ciphertext.is_empty() {
        return Ok(String::new());
    }

    let decryptor = Aes128CbcDec::new((&key[..16]).into(), (&iv).into());
    let mut buf = ciphertext.to_vec();

    let decrypted = decryptor
        .decrypt_padded_mut::<cbc::cipher::block_padding::Pkcs7>(&mut buf)
        .map_err(|e| {
            ExtractError::DatabaseError(format!("Linux AES-CBC decryption error: {:?}", e))
        })?;

    Ok(String::from_utf8_lossy(decrypted).to_string())
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
}
