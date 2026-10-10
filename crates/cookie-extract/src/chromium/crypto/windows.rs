//! Windows DPAPI and App-Bound detection for Chromium.

use crate::errors::ExtractError;

pub fn get_windows_key() -> Result<Vec<u8>, ExtractError> {
    // On Windows, keys are retrieved from Local State JSON os_crypt.encrypted_key
    // If encrypted with App-Bound (Chrome 127+), it cannot be decrypted by external CLI
    // For now, return AppBoundRestricted if DPAPI / App-Bound fails
    Err(ExtractError::AppBoundRestricted)
}
