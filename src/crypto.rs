use aes::cipher::{
    BlockDecryptMut, BlockEncryptMut, KeyIvInit,
    block_padding::{Pkcs7, UnpadError},
};
use sha1::{Digest, Sha1};

type Aes256CbcEnc = cbc::Encryptor<aes::Aes256Enc>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256Dec>;

pub type CryptoResult<T> = core::result::Result<T, Error>;

const SALT: &[u8] = &[
    0x52, 0xA6, 0x42, 0x57, 0x92, 0x33, 0xB3, 0x6C, 0xF2, 0x6E, 0x62, 0xED, 0x7C,
];
const PROGRESS_PASSWORD: &[u8] = b"56SA%FG42Dv5#4aG67f2";
const CONTRAPTION_PASSWORD: &[u8] = b"3b91A049Ca7HvSjhxT35";

// --- Public API ---

pub fn encrypt_contraption(buffer: &[u8]) -> Vec<u8> {
    let (key, iv) = derive_key_iv(CONTRAPTION_PASSWORD);
    aes_encrypt(&key, &iv, buffer)
}

pub fn decrypt_contraption(buffer: &[u8]) -> CryptoResult<Vec<u8>> {
    let (key, iv) = derive_key_iv(CONTRAPTION_PASSWORD);
    aes_decrypt(&key, &iv, buffer)
}

pub fn encrypt_progress(buffer: &[u8]) -> Vec<u8> {
    let (key, iv) = derive_key_iv(PROGRESS_PASSWORD);
    let cipher_buffer = aes_encrypt(&key, &iv, buffer);
    let sha1_buffer = sha1_checksum(&cipher_buffer);

    [sha1_buffer, cipher_buffer].concat()
}

pub fn decrypt_progress(buffer: &[u8]) -> CryptoResult<Vec<u8>> {
    if buffer.len() < 20 {
        return Err(Error::Sha1HashLengthError(buffer.len()));
    }
    let (expected_checksum, cipher_slice) = buffer.split_at(20);

    let got_checksum = sha1_checksum(cipher_slice);

    if expected_checksum != got_checksum.as_slice() {
        return Err(Error::Sha1ChecksumError(
            expected_checksum.to_vec(),
            got_checksum,
        ));
    }

    let (key, iv) = derive_key_iv(PROGRESS_PASSWORD);
    aes_decrypt(&key, &iv, cipher_slice)
}

// --- Internal Helpers ---

fn aes_encrypt(key: &[u8], iv: &[u8], buffer: &[u8]) -> Vec<u8> {
    Aes256CbcEnc::new(key.into(), iv.into()).encrypt_padded_vec_mut::<Pkcs7>(buffer)
}

fn aes_decrypt(key: &[u8], iv: &[u8], buffer: &[u8]) -> CryptoResult<Vec<u8>> {
    Ok(Aes256CbcDec::new(key.into(), iv.into()).decrypt_padded_vec_mut::<Pkcs7>(buffer)?)
}

fn sha1_checksum(buffer: &[u8]) -> Vec<u8> {
    Sha1::new_with_prefix(buffer).finalize().to_vec()
}

fn derive_key_iv(password: &[u8]) -> ([u8; 32], [u8; 16]) {
    let bytes = pbkdf2::pbkdf2_hmac_array::<Sha1, 48>(password, SALT, 1000);
    (
        bytes[..32].try_into().unwrap(), // Key
        bytes[32..].try_into().unwrap(), // IV
    )
}

// --- Error Definitions ---

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Data too short: expected at least 20 bytes, got {0}")]
    Sha1HashLengthError(usize),

    #[error("Checksum mismatch: expected {0:?}, got {1:?}")]
    Sha1ChecksumError(Vec<u8>, Vec<u8>),

    #[error("AES decryption/padding error: {0}")]
    CbcPaddingError(#[from] UnpadError),
}

// --- Tests ---
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_contraption() {
        let data = b"Test contraption data";
        let encrypted = encrypt_contraption(data);
        let decrypted = decrypt_contraption(&encrypted).unwrap();
        assert_eq!(data, &decrypted[..]);
    }

    #[test]
    fn test_encrypt_decrypt_progress() {
        let data = b"Test progress data";
        let encrypted = encrypt_progress(data);
        let decrypted = decrypt_progress(&encrypted).unwrap();
        assert_eq!(data, &decrypted[..]);
    }

    #[test]
    fn test_decrypt_bad_checksum() {
        let data = b"Data";
        let mut encrypted = encrypt_progress(data);
        encrypted[0] = encrypted[0].wrapping_add(1); // Corrupt the data
        assert!(matches!(
            decrypt_progress(&encrypted),
            Err(Error::Sha1ChecksumError(_, _))
        ));
    }
}
