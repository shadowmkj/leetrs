//! Linux SecretService / fallback password decryption for Chromium.

use crate::errors::ExtractError;
use aes::Aes128;
use cbc::cipher::{BlockDecryptMut, KeyIvInit};
use pbkdf2::pbkdf2_hmac;
use sha1::Sha1;

type Aes128CbcDec = cbc::Decryptor<Aes128>;

pub fn get_linux_key() -> Result<Vec<u8>, ExtractError> {
    // Linux Chromium uses "peanuts" as default fallback password if SecretService / GNOME keyring is not active
    let password = "peanuts";
    let mut key = [0u8; 16];
    pbkdf2_hmac::<Sha1>(password.as_bytes(), b"saltysalt", 1, &mut key);
    Ok(key.to_vec())
}

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
