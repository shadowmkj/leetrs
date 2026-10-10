//! Windows DPAPI and App-Bound detection for Chromium.

use crate::errors::ExtractError;

pub fn get_windows_key() -> Result<Vec<u8>, ExtractError> {
    // On Windows, keys are retrieved from Local State JSON os_crypt.encrypted_key.
    // When encrypted with App-Bound Encryption (Chrome 127+), it cannot be decrypted by external CLI tools.
    // Return AppBoundRestricted so the user is guided to manual paste.
    Err(ExtractError::AppBoundRestricted)
}

pub fn decrypt_v10_windows(_key: &[u8], _encrypted_value: &[u8]) -> Result<String, ExtractError> {
    Err(ExtractError::AppBoundRestricted)
}
