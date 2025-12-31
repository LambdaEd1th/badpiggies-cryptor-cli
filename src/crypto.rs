use aes::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit, block_padding::Pkcs7};
use sha1::{Digest, Sha1};

use crate::{constants, errors::Error};

type Aes256CbcEnc = cbc::Encryptor<aes::Aes256Enc>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256Dec>;

pub type CryptoResult<T> = core::result::Result<T, Error>;

// --- Public API ---

pub fn encrypt_contraption(buffer: &[u8]) -> Vec<u8> {
    let (key, iv) = derive_key_iv(constants::CONTRAPTION_PWD);
    aes_encrypt(&key, &iv, buffer)
}

pub fn decrypt_contraption(buffer: &[u8]) -> CryptoResult<Vec<u8>> {
    let (key, iv) = derive_key_iv(constants::CONTRAPTION_PWD);
    aes_decrypt(&key, &iv, buffer)
}

pub fn encrypt_progress(buffer: &[u8]) -> Vec<u8> {
    let (key, iv) = derive_key_iv(constants::PROGRESS_PWD);
    let mut cipher_buffer = aes_encrypt(&key, &iv, buffer);

    // Calculate checksum and prepend it to the data
    let mut final_data = sha1_checksum(&cipher_buffer);
    final_data.append(&mut cipher_buffer);
    final_data
}

pub fn decrypt_progress(buffer: &[u8]) -> CryptoResult<Vec<u8>> {
    // Check if data is long enough to contain the header
    if buffer.len() < constants::SHA1_HEADER_LEN {
        return Err(Error::Sha1HashLength(buffer.len()));
    }

    // Split the buffer into checksum header and actual encrypted data
    let (expected_checksum, cipher_slice) = buffer.split_at(constants::SHA1_HEADER_LEN);
    let got_checksum = sha1_checksum(cipher_slice);

    // Verify integrity
    if expected_checksum != got_checksum.as_slice() {
        return Err(Error::Sha1Checksum(
            expected_checksum.to_vec(),
            got_checksum,
        ));
    }

    let (key, iv) = derive_key_iv(constants::PROGRESS_PWD);
    aes_decrypt(&key, &iv, cipher_slice)
}

// --- Internal Helpers ---

fn aes_encrypt(key: &[u8], iv: &[u8], buffer: &[u8]) -> Vec<u8> {
    Aes256CbcEnc::new(key.into(), iv.into()).encrypt_padded_vec_mut::<Pkcs7>(buffer)
}

fn aes_decrypt(key: &[u8], iv: &[u8], buffer: &[u8]) -> CryptoResult<Vec<u8>> {
    Aes256CbcDec::new(key.into(), iv.into())
        .decrypt_padded_vec_mut::<Pkcs7>(buffer)
        .map_err(Error::from)
}

fn sha1_checksum(buffer: &[u8]) -> Vec<u8> {
    Sha1::new_with_prefix(buffer).finalize().to_vec()
}

fn derive_key_iv(password: &[u8]) -> ([u8; constants::KEY_LEN], [u8; constants::IV_LEN]) {
    // Generate enough bytes for both Key and IV using PBKDF2
    let bytes = pbkdf2::pbkdf2_hmac_array::<Sha1, { constants::DERIVED_LEN }>(
        password,
        constants::SALT,
        constants::PBKDF2_ITERATIONS,
    );

    let (key, iv) = bytes.split_at(constants::KEY_LEN);

    (
        key.try_into().expect("Slice length must match KEY_LEN"),
        iv.try_into().expect("Slice length must match IV_LEN"),
    )
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
