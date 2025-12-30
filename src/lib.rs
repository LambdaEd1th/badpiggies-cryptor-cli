pub mod cli;
pub mod crypto;

use anyhow::Result;
use cli::FileTypes;

/// Core processing logic: dispatches to specific crypto functions based on configuration.
/// Pure function: Input Bytes -> Output Bytes (or Error).
/// Does NOT handle file I/O.
pub fn process_data(file_type: &FileTypes, is_encrypt: bool, data: &[u8]) -> Result<Vec<u8>> {
    match (file_type, is_encrypt) {
        // Progress.dat: Uses AES + SHA1 Checksum
        (FileTypes::Progress, true) => Ok(crypto::encrypt_progress(data)),
        (FileTypes::Progress, false) => Ok(crypto::decrypt_progress(data)?),

        // .contraption: Uses AES only
        (FileTypes::Contraption, true) => Ok(crypto::encrypt_contraption(data)),
        (FileTypes::Contraption, false) => Ok(crypto::decrypt_contraption(data)?),
    }
}
