use aes::cipher::{
    BlockDecryptMut, BlockEncryptMut, KeyIvInit,
    block_padding::{Pkcs7, UnpadError},
};
use sha1::{Digest, Sha1};

type Aes256CbcEnc = cbc::Encryptor<aes::Aes256Enc>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256Dec>;

pub type CryptoResult<T> = core::result::Result<T, Error>;

mod secrets {
    pub const SALT: &[u8] = &[
        0x52, 0xA6, 0x42, 0x57, 0x92, 0x33, 0xB3, 0x6C, 0xF2, 0x6E, 0x62, 0xED, 0x7C,
    ];
    pub const PROGRESS_PWD: &[u8] = b"56SA%FG42Dv5#4aG67f2";
    pub const CONTRAPTION_PWD: &[u8] = b"3b91A049Ca7HvSjhxT35";
}

// --- Public API ---

pub fn encrypt_contraption(buffer: &[u8]) -> Vec<u8> {
    let (key, iv) = derive_key_iv(secrets::CONTRAPTION_PWD);
    aes_encrypt(&key, &iv, buffer)
}

pub fn decrypt_contraption(buffer: &[u8]) -> CryptoResult<Vec<u8>> {
    let (key, iv) = derive_key_iv(secrets::CONTRAPTION_PWD);
    aes_decrypt(&key, &iv, buffer)
}

pub fn encrypt_progress(buffer: &[u8]) -> Vec<u8> {
    let (key, iv) = derive_key_iv(secrets::PROGRESS_PWD);
    let mut cipher_buffer = aes_encrypt(&key, &iv, buffer);
    
    let mut final_data = sha1_checksum(&cipher_buffer);
    final_data.append(&mut cipher_buffer);
    final_data
}

pub fn decrypt_progress(buffer: &[u8]) -> CryptoResult<Vec<u8>> {
    if buffer.len() < 20 {
        return Err(Error::Sha1HashLength(buffer.len()));
    }
    
    let (expected_checksum, cipher_slice) = buffer.split_at(20);
    let got_checksum = sha1_checksum(cipher_slice);

    if expected_checksum != got_checksum.as_slice() {
        return Err(Error::Sha1Checksum(
            expected_checksum.to_vec(),
            got_checksum,
        ));
    }

    let (key, iv) = derive_key_iv(secrets::PROGRESS_PWD);
    aes_decrypt(&key, &iv, cipher_slice)
}

// --- Internal Helpers ---

fn aes_encrypt(key: &[u8], iv: &[u8], buffer: &[u8]) -> Vec<u8> {
    Aes256CbcEnc::new(key.into(), iv.into())
        .encrypt_padded_vec_mut::<Pkcs7>(buffer)
}

fn aes_decrypt(key: &[u8], iv: &[u8], buffer: &[u8]) -> CryptoResult<Vec<u8>> {
    Aes256CbcDec::new(key.into(), iv.into())
        .decrypt_padded_vec_mut::<Pkcs7>(buffer)
        .map_err(Error::from)
}

fn sha1_checksum(buffer: &[u8]) -> Vec<u8> {
    Sha1::new_with_prefix(buffer).finalize().to_vec()
}

fn derive_key_iv(password: &[u8]) -> ([u8; 32], [u8; 16]) {
    let bytes = pbkdf2::pbkdf2_hmac_array::<Sha1, 48>(password, secrets::SALT, 1000);
    let (key, iv) = bytes.split_at(32);
    
    (
        key.try_into().expect("Slice length must match"), 
        iv.try_into().expect("Slice length must match")
    )
}

// --- Error Definitions ---
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Data too short: expected at least 20 bytes (hash header), got {0}")]
    Sha1HashLength(usize),

    #[error("Checksum mismatch: file may be corrupted or modified.\nExpected: {0:x?}\nCalculated: {1:x?}")]
    Sha1Checksum(Vec<u8>, Vec<u8>),

    #[error("AES decryption/padding error: {0}")]
    CbcPadding(#[from] UnpadError),
}

// --- Tests ---
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contraption_cycle() {
        let original = b"Test Vehicle Blueprint";
        let encrypted = encrypt_contraption(original);
        let decrypted = decrypt_contraption(&encrypted).expect("Decryption failed");
        assert_eq!(original, &decrypted[..]);
    }

    #[test]
    fn test_progress_cycle() {
        let original = b"Test Game Save Progress";
        let encrypted = encrypt_progress(original);
        let decrypted = decrypt_progress(&encrypted).expect("Decryption failed");
        assert_eq!(original, &decrypted[..]);
    }
}