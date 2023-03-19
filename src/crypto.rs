use std::{
    error::Error,
    fmt::{self, Display},
};

use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use sha1::{Digest, Sha1};

type Aes256CbcEnc = cbc::Encryptor<aes::Aes256Enc>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256Dec>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cryptor<'a, 'b> {
    pub password: &'a [u8],
    pub salt: &'b [u8],
}

impl<'a, 'b> Cryptor<'a, 'b> {
    pub fn new(password: &'a [u8], salt: &'b [u8]) -> Cryptor<'a, 'b> {
        Cryptor {
            password,
            salt,
        }
    }

    pub fn encrypt(&self, buffer: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        let (key, iv) = Self::rfc2898_derive_bytes(&self)?;
        let encryptor = Aes256CbcEnc::new(&key.into(), &iv.into());
        let cipher_buffer = encryptor.encrypt_padded_vec_mut::<Pkcs7>(buffer);
        Ok(cipher_buffer)
    }

    pub fn decrypt(&self, buffer: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        let (key, iv) = Self::rfc2898_derive_bytes(&self)?;
        let decryptor = Aes256CbcDec::new(&key.into(), &iv.into());
        let plain_buffer = decryptor.decrypt_padded_vec_mut::<Pkcs7>(buffer)?;
        Ok(plain_buffer)
    }

    pub fn sha1_hash(buffer: &[u8]) -> Vec<u8> {
        Sha1::new_with_prefix(buffer).finalize().to_vec()
    }

    fn rfc2898_derive_bytes(&self) -> Result<([u8; 32], [u8; 16]), Box<dyn Error>> {
        let bytes = pbkdf2::pbkdf2_hmac_array::<Sha1, 48>(self.password, self.salt, 1000);
        Ok((bytes[..32].try_into()?, bytes[32..].try_into()?))
    }
}

/// Failed hash error.
#[derive(Clone, Copy, Debug)]
pub struct Sha1HashError;

impl Display for Sha1HashError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str("SHA-1 check failed")
    }
}

impl Error for Sha1HashError {}
