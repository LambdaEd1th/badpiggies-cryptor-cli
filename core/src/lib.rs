pub mod constants;
pub mod crypto;
pub mod errors;

pub mod mode {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum CryptoMode {
        Encrypt,
        Decrypt,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Categories {
    Progress,
    Contraption,
}

use crate::mode::CryptoMode;
use anyhow::Result;

/// Core processing logic: dispatches to specific crypto functions based on configuration.
/// Pure function: Input Bytes -> Output Bytes (or Error).
/// Does NOT handle file I/O.
pub fn process_data(category: &Categories, mode: CryptoMode, data: &[u8]) -> Result<Vec<u8>> {
    match (category, mode) {
        // Progress.dat: Uses AES + SHA1 Checksum
        (Categories::Progress, CryptoMode::Encrypt) => Ok(crypto::encrypt_progress(data)?),
        (Categories::Progress, CryptoMode::Decrypt) => Ok(crypto::decrypt_progress(data)?),

        // .contraption: Uses AES only
        (Categories::Contraption, CryptoMode::Encrypt) => Ok(crypto::encrypt_contraption(data)?),
        (Categories::Contraption, CryptoMode::Decrypt) => Ok(crypto::decrypt_contraption(data)?),
    }
}
