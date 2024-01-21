use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use sha1::{Digest, Sha1};

type Aes256CbcEnc = cbc::Encryptor<aes::Aes256Enc>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256Dec>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cryptor<'cryptor> {
    file_buffer: &'cryptor [u8],
}

impl<'cryptor> Cryptor<'cryptor> {
    const SALT: &'cryptor [u8] = &[
        0x52, 0xA6, 0x42, 0x57, 0x92, 0x33, 0xB3, 0x6C, 0xF2, 0x6E, 0x62, 0xED, 0x7C,
    ];
    const PROGRESS_PASSWORD: &'cryptor [u8] = b"56SA%FG42Dv5#4aG67f2";
    const CONTRAPTION_PASSWORD: &'cryptor [u8] = b"3b91A049Ca7HvSjhxT35";

    /// Creates a new [`Cryptor`].
    pub fn new(file_buffer: &'cryptor [u8]) -> Self {
        Self { file_buffer }
    }

    pub fn encrypt_contraption(&self) -> Vec<u8> {
        let (key, iv) = self.rfc2898_derive_bytes(Self::CONTRAPTION_PASSWORD);
        let cipher_buffer = self.aes_encrypt(&key, &iv, self.file_buffer);
        cipher_buffer
    }

    pub fn decrypt_contraption(&self) -> Result<Vec<u8>, CryptorError> {
        let (key, iv) = self.rfc2898_derive_bytes(Self::CONTRAPTION_PASSWORD);
        let plain_buffer = self.aes_decrypt(&key, &iv, self.file_buffer)?;
        Ok(plain_buffer)
    }

    pub fn encrypt_progress(&self) -> Vec<u8> {
        let (key, iv) = self.rfc2898_derive_bytes(Self::PROGRESS_PASSWORD);
        let cipher_buffer = self.aes_encrypt(&key, &iv, self.file_buffer);
        let sha1_buffer = self.sha1_hash(&cipher_buffer);
        [sha1_buffer, cipher_buffer].concat()
    }

    pub fn decrypt_progress(&self) -> Result<Vec<u8>, CryptorError> {
        let (sha1_slice, cipher_slice) = match self.file_buffer.len() >= 20 {
            true => self.file_buffer.split_at(20),
            false => {
                return Err(CryptorError::Sha1HashError(
                    "SHA-1 contents too short".to_owned(),
                ));
            }
        };
        let cipher_buffer: Vec<u8> = match sha1_slice == self.sha1_hash(cipher_slice) {
            true => cipher_slice.to_owned(),
            false => {
                return Err(CryptorError::Sha1HashError(
                    "SHA-1 checking failed".to_owned(),
                ));
            }
        };
        let (key, iv) = self.rfc2898_derive_bytes(Self::PROGRESS_PASSWORD);
        let plain_buffer = self.aes_decrypt(&key, &iv, &cipher_buffer)?;
        Ok(plain_buffer)
    }

    fn aes_encrypt(&self, key: &[u8], iv: &[u8], buffer: &[u8]) -> Vec<u8> {
        let encryptor = Aes256CbcEnc::new(key.into(), iv.into());
        let cipher = encryptor.encrypt_padded_vec_mut::<Pkcs7>(buffer);
        cipher
    }

    fn aes_decrypt(&self, key: &[u8], iv: &[u8], buffer: &[u8]) -> Result<Vec<u8>, CryptorError> {
        let decryptor = Aes256CbcDec::new(key.into(), iv.into());
        let plain = decryptor
            .decrypt_padded_vec_mut::<Pkcs7>(buffer)
            .map_err(|e| CryptorError::AesCryptoError(e.to_string()))?;
        Ok(plain)
    }

    fn sha1_hash(&self, buffer: &[u8]) -> Vec<u8> {
        Sha1::new_with_prefix(buffer).finalize().to_vec()
    }

    fn rfc2898_derive_bytes(&self, password: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let bytes = pbkdf2::pbkdf2_hmac_array::<Sha1, 48>(password, Self::SALT, 1000);
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
