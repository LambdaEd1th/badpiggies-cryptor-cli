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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cryptor;

impl Cryptor {
    /// Creates a new [`Cryptor`].
    pub fn new() -> Self {
        Self {}
    }

    pub fn encrypt_contraption(&self, buffer: &[u8]) -> Vec<u8> {
        let (key, iv) = self.rfc2898_derive_bytes(CONTRAPTION_PASSWORD);
        let cipher_buffer = self.aes_encrypt(&key, &iv, buffer);
        cipher_buffer
    }

    pub fn decrypt_contraption(&self, buffer: &[u8]) -> Result<Vec<u8>> {
        let (key, iv) = self.rfc2898_derive_bytes(CONTRAPTION_PASSWORD);
        let plain_buffer = self.aes_decrypt(&key, &iv, buffer)?;
        Ok(plain_buffer)
    }

    pub fn encrypt_progress(&self, buffer: &[u8]) -> Vec<u8> {
        let (key, iv) = self.rfc2898_derive_bytes(PROGRESS_PASSWORD);
        let cipher_buffer = self.aes_encrypt(&key, &iv, buffer);
        let sha1_buffer = self.sha1_hash(&cipher_buffer);
        [sha1_buffer, cipher_buffer].concat()
    }

    pub fn decrypt_progress(&self, buffer: &[u8]) -> Result<Vec<u8>> {
        let input_file_len = buffer.len();
        let (sha1_slice, cipher_slice) = if input_file_len >= 20 {
            buffer.split_at(20)
        } else {
            return Err(Error::Sha1HashLengthError(input_file_len));
        };

        let sha1_hash = self.sha1_hash(cipher_slice);
        let cipher_buffer: Vec<u8> = if sha1_slice == &sha1_hash {
            cipher_slice.to_vec()
        } else {
            return Err(Error::Sha1HashCheckError(sha1_slice.to_vec(), sha1_hash));
        };

        let (key, iv) = self.rfc2898_derive_bytes(PROGRESS_PASSWORD);
        let plain_buffer = self.aes_decrypt(&key, &iv, &cipher_buffer)?;
        Ok(plain_buffer)
    }

    fn aes_encrypt(&self, key: &[u8], iv: &[u8], buffer: &[u8]) -> Vec<u8> {
        let encryptor = Aes256CbcEnc::new(key.into(), iv.into());
        let cipher = encryptor.encrypt_padded_vec_mut::<Pkcs7>(buffer);
        cipher
    }

    fn aes_decrypt(&self, key: &[u8], iv: &[u8], buffer: &[u8]) -> Result<Vec<u8>> {
        let decryptor = Aes256CbcDec::new(key.into(), iv.into());
        let plain = decryptor.decrypt_padded_vec_mut::<Pkcs7>(buffer)?;
        Ok(plain)
    }

    fn sha1_hash(&self, buffer: &[u8]) -> Vec<u8> {
        Sha1::new_with_prefix(buffer).finalize().to_vec()
    }

    fn rfc2898_derive_bytes(&self, password: &[u8]) -> ([u8; 32], [u8; 16]) {
        let bytes = pbkdf2::pbkdf2_hmac_array::<Sha1, 48>(password, SALT, 1000);
        let (mut key, mut iv) = ([0u8; 32], [0u8; 16]);
        key.clone_from_slice(&bytes[..32]);
        iv.clone_from_slice(&bytes[32..]);
        (key, iv)
    }
}

#[derive(Debug)]
pub enum Error {
    // Failed hash error
    Sha1HashLengthError(usize),
    Sha1HashCheckError(Vec<u8>, Vec<u8>),
    AesCryptoError(UnpadError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Sha1HashLengthError(len) => write!(f, "Sha1HashLengthError: {len}"),
            Self::Sha1HashCheckError(sha1_slice, sha1_hash) => {
                write!(f, "Sha1HashCheckError: {sha1_slice:#X?} {sha1_hash:#X?}")
            }
            Self::AesCryptoError(err) => write!(f, "AesCryptoError: {err}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            Self::Sha1HashLengthError(_) => None,
            Self::Sha1HashCheckError(_, _) => None,
            Self::AesCryptoError(err) => Some(err),
        }
    }
}

impl From<UnpadError> for Error {
    fn from(err: UnpadError) -> Self {
        Self::AesCryptoError(err)
    }
}
