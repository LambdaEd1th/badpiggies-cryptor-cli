use aes::cipher::block_padding::UnpadError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Data too short: expected at least 20 bytes (hash header), got {0}")]
    Sha1HashLength(usize),

    #[error(
        "Checksum mismatch: file may be corrupted or modified.\nExpected: {0:x?}\nCalculated: {1:x?}"
    )]
    Sha1Checksum(Vec<u8>, Vec<u8>),

    #[error("AES decryption/padding error: {0}")]
    CbcPadding(#[from] UnpadError),
}
