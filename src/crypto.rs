use std::{
    error::Error as StdError,
    fmt::{Display as FmtDisplay, Formatter as FmtFormatter, Result as FmtResult},
};

use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use sha1::{Digest, Sha1};

type Aes256CbcEnc = cbc::Encryptor<aes::Aes256Enc>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256Dec>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cryptor<'cryptor> {
    password: &'cryptor [u8],
    salt: &'cryptor [u8],
}

impl<'cryptor> Cryptor<'cryptor> {
    /// Creates a new [`Cryptor`].
    pub fn new(password: &'cryptor [u8], salt: &'cryptor [u8]) -> Self {
        Self { password, salt }
    }

    pub fn encrypt(&self, buffer: &[u8]) -> Result<Vec<u8>, Box<dyn StdError>> {
        let (key, iv) = Self::rfc2898_derive_bytes(&self)?;
        Ok(Self::aes_encrypt(&self, &key, &iv, buffer)?)
    }

    pub fn decrypt(&self, buffer: &[u8]) -> Result<Vec<u8>, Box<dyn StdError>> {
        let (key, iv) = Self::rfc2898_derive_bytes(&self)?;
        Ok(Self::aes_decrypt(&self, &key, &iv, buffer)?)
    }

    pub fn encrypt_with_sha1_hash(&self, buffer: &[u8]) -> Result<Vec<u8>, Box<dyn StdError>> {
        let cipher_buffer = self.encrypt(&buffer)?;
        let sha1_buffer = Self::sha1_hash(&cipher_buffer);
        Ok([sha1_buffer, cipher_buffer].concat())
    }

    pub fn decrypt_with_sha1_hash(&self, buffer: &[u8]) -> Result<Vec<u8>, Box<dyn StdError>> {
        if buffer.len() < 20 {
            return Err(Box::new(CryptorError::Sha1HashError(
                "SHA-1 contents too short".to_owned(),
            )));
        }
        let (sha1_slice, cipher_slice) = buffer.split_at(20);
        if sha1_slice != Self::sha1_hash(cipher_slice) {
            return Err(Box::new(CryptorError::Sha1HashError(
                "SHA-1 checking failed".to_owned(),
            )));
        }
        let cipher_buffer: Vec<u8> = cipher_slice.to_vec();
        Ok(Self::decrypt(&self, &cipher_buffer)?)
    }

    fn aes_encrypt(
        &self,
        key: &[u8; 32],
        iv: &[u8; 16],
        buffer: &[u8],
    ) -> Result<Vec<u8>, Box<dyn StdError>> {
        let encryptor = Aes256CbcEnc::new(key.into(), iv.into());
        Ok(encryptor.encrypt_padded_vec_mut::<Pkcs7>(buffer))
    }

    fn aes_decrypt(
        &self,
        key: &[u8; 32],
        iv: &[u8; 16],
        buffer: &[u8],
    ) -> Result<Vec<u8>, Box<dyn StdError>> {
        let decryptor = Aes256CbcDec::new(key.into(), iv.into());
        Ok(decryptor.decrypt_padded_vec_mut::<Pkcs7>(buffer)?)
    }

    fn sha1_hash(buffer: &[u8]) -> Vec<u8> {
        Sha1::new_with_prefix(buffer).finalize().to_vec()
    }

    fn rfc2898_derive_bytes(&self) -> Result<([u8; 32], [u8; 16]), Box<dyn StdError>> {
        let bytes = pbkdf2::pbkdf2_hmac_array::<Sha1, 48>(self.password, self.salt, 1000);
        Ok((bytes[..32].try_into()?, bytes[32..].try_into()?))
    }
}

#[derive(Debug)]
pub enum CryptorError {
    // Failed hash error
    Sha1HashError(String),
}

impl FmtDisplay for CryptorError {
    fn fmt(&self, f: &mut FmtFormatter<'_>) -> FmtResult {
        match self {
            Self::Sha1HashError(s) => write!(f, "Sha1HashError: {}", s),
        }
    }
}

impl StdError for CryptorError {}
