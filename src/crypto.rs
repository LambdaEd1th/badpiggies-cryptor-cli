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

    pub fn encrypt(&self, buffer: &[u8]) -> Result<Vec<u8>, CryptorError> {
        let (key, iv) = self.rfc2898_derive_bytes();
        let cipher = Self::aes_encrypt(&self, &key, &iv, buffer)
            .map_err(|e| CryptorError::AesCryptoError(e.to_string()))?;
        Ok(cipher)
    }

    pub fn decrypt(&self, buffer: &[u8]) -> Result<Vec<u8>, CryptorError> {
        let (key, iv) = self.rfc2898_derive_bytes();
        Ok(Self::aes_decrypt(&self, &key, &iv, buffer)?)
    }

    pub fn encrypt_with_sha1_hash(&self, buffer: &[u8]) -> Result<Vec<u8>, CryptorError> {
        let cipher_buffer = self
            .encrypt(&buffer)
            .map_err(|e| CryptorError::Sha1HashError(e.to_string()))?;
        let sha1_buffer = Self::sha1_hash(&cipher_buffer);
        Ok([sha1_buffer, cipher_buffer].concat())
    }

    pub fn decrypt_with_sha1_hash(&self, buffer: &[u8]) -> Result<Vec<u8>, CryptorError> {
        let (sha1_slice, cipher_slice) = match buffer.len() >= 20 {
            true => buffer.split_at(20),
            false => {
                return Err(CryptorError::Sha1HashError(
                    "SHA-1 contents too short".to_owned(),
                ));
            }
        };
        let cipher_buffer: Vec<u8> = match sha1_slice == Self::sha1_hash(cipher_slice) {
            true => cipher_slice.to_owned(),
            false => {
                return Err(CryptorError::Sha1HashError(
                    "SHA-1 checking failed".to_owned(),
                ));
            },
        };
        Ok(Self::decrypt(&self, &cipher_buffer)?)
    }

    fn aes_encrypt(&self, key: &[u8], iv: &[u8], buffer: &[u8]) -> Result<Vec<u8>, CryptorError> {
        let encryptor = Aes256CbcEnc::new(key.into(), iv.into());
        Ok(encryptor.encrypt_padded_vec_mut::<Pkcs7>(buffer))
    }

    fn aes_decrypt(&self, key: &[u8], iv: &[u8], buffer: &[u8]) -> Result<Vec<u8>, CryptorError> {
        let decryptor = Aes256CbcDec::new(key.into(), iv.into());
        let plain = decryptor
            .decrypt_padded_vec_mut::<Pkcs7>(buffer)
            .map_err(|e| CryptorError::AesCryptoError(e.to_string()))?;
        Ok(plain)
    }

    fn sha1_hash(buffer: &[u8]) -> Vec<u8> {
        Sha1::new_with_prefix(buffer).finalize().to_vec()
    }

    fn rfc2898_derive_bytes(&self) -> (Vec<u8>, Vec<u8>) {
        let bytes = pbkdf2::pbkdf2_hmac_array::<Sha1, 48>(self.password, self.salt, 1000);
        let (key, iv) = bytes.split_at(32);
        (key.to_owned(), iv.to_owned())
    }
}

#[derive(Debug)]
pub enum CryptorError {
    // Failed hash error
    Sha1HashError(String),
    AesCryptoError(String),
}

impl std::fmt::Display for CryptorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sha1HashError(s) => write!(f, "Sha1HashError: {}", s),
            Self::AesCryptoError(s) => write!(f, "AesCryptoError: {}", s),
        }
    }
}

impl std::error::Error for CryptorError {}
