use aes::cipher::{
    block_padding::{Pkcs7, UnpadError},
    BlockDecryptMut, BlockEncryptMut, KeyIvInit,
};
use sha1::{Digest, Sha1};

type Aes256CbcEnc = cbc::Encryptor<aes::Aes256Enc>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256Dec>;

pub type Result<T> = core::result::Result<T, Error>;

const SALT: &[u8] = &[
    0x52, 0xA6, 0x42, 0x57, 0x92, 0x33, 0xB3, 0x6C, 0xF2, 0x6E, 0x62, 0xED, 0x7C,
];
const PROGRESS_PASSWORD: &[u8] = b"56SA%FG42Dv5#4aG67f2";
const CONTRAPTION_PASSWORD: &[u8] = b"3b91A049Ca7HvSjhxT35";

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Cryptor;

impl Cryptor {
    pub fn new() -> Self {
        Self
    }

    pub fn encrypt_contraption(&self, buffer: &[u8]) -> Vec<u8> {
        let (key, iv) = self.derive_key_iv(CONTRAPTION_PASSWORD);
        self.aes_encrypt(&key, &iv, buffer)
    }

    pub fn decrypt_contraption(&self, buffer: &[u8]) -> Result<Vec<u8>> {
        let (key, iv) = self.derive_key_iv(CONTRAPTION_PASSWORD);
        self.aes_decrypt(&key, &iv, buffer)
    }

    pub fn encrypt_progress(&self, buffer: &[u8]) -> Vec<u8> {
        let (key, iv) = self.derive_key_iv(PROGRESS_PASSWORD);
        let cipher_buffer = self.aes_encrypt(&key, &iv, buffer);
        let sha1_buffer = self.sha1_checksum(&cipher_buffer);
        [sha1_buffer, cipher_buffer].concat()
    }

    pub fn decrypt_progress(&self, buffer: &[u8]) -> Result<Vec<u8>> {
        // Ensure the buffer is at least 20 bytes to contain the SHA1 hash
        if buffer.len() < 20 {
            return Err(Error::Sha1HashLengthError(buffer.len()));
        }
        let (sha1_slice, cipher_slice) = buffer.split_at(20);

        let got_checksum = self.sha1_checksum(cipher_slice);
        let expected_checksum = sha1_slice.to_vec();

        if expected_checksum != got_checksum {
            return Err(Error::Sha1ChecksumError(
                expected_checksum,
                got_checksum,
            ));
        }

        let (key, iv) = self.derive_key_iv(PROGRESS_PASSWORD);
        self.aes_decrypt(&key, &iv, cipher_slice)
    }

    fn aes_encrypt(&self, key: &[u8], iv: &[u8], buffer: &[u8]) -> Vec<u8> {
        Aes256CbcEnc::new(key.into(), iv.into()).encrypt_padded_vec_mut::<Pkcs7>(buffer)
    }

    fn aes_decrypt(&self, key: &[u8], iv: &[u8], buffer: &[u8]) -> Result<Vec<u8>> {
        Ok(Aes256CbcDec::new(key.into(), iv.into()).decrypt_padded_vec_mut::<Pkcs7>(buffer)?)
    }

    fn sha1_checksum(&self, buffer: &[u8]) -> Vec<u8> {
        Sha1::new_with_prefix(buffer).finalize().to_vec()
    }

    fn derive_key_iv(&self, password: &[u8]) -> ([u8; 32], [u8; 16]) {
        let bytes = pbkdf2::pbkdf2_hmac_array::<Sha1, 48>(password, SALT, 1000);
        (
            bytes[..32].try_into().unwrap(),
            bytes[32..].try_into().unwrap(),
        )
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("SHA1 hash length error: expected 20 bytes, got {0}")]
    Sha1HashLengthError(usize),
    #[error("SHA1 checksum mismatch: expected {0:?}, got {1:?}")]
    Sha1ChecksumError(Vec<u8>, Vec<u8>),
    #[error("AES encryption/decryption error: {0}")]
    CbcPaddingError(#[from] UnpadError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_contraption() {
        let cryptor = Cryptor::new();
        let data = b"Test contraption data";
        let encrypted = cryptor.encrypt_contraption(data);
        let decrypted = cryptor.decrypt_contraption(&encrypted).unwrap();
        assert_eq!(data, &decrypted[..]);
    }

    #[test]
    fn test_encrypt_decrypt_progress() {
        let cryptor = Cryptor::new();
        let data = b"Test progress data";
        let encrypted = cryptor.encrypt_progress(data);
        let decrypted = cryptor.decrypt_progress(&encrypted).unwrap();
        assert_eq!(data, &decrypted[..]);
    }
}
