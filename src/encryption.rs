use std::result;

use aes::cipher::block_padding::{Pkcs7, UnpadError};
use aes::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use aes::Aes256;
use cbc::{Decryptor, Encryptor};

type Aes256CbcEnc = Encryptor<Aes256>;
type Aes256CbcDec = Decryptor<Aes256>;

pub fn encrypt256(key: [u8; 32], iv: [u8; 16], msg: &[u8]) -> Vec<u8> {
    let cipher = Aes256CbcEnc::new(&key.into(), &iv.into());
    cipher.encrypt_padded_vec_mut::<Pkcs7>(msg)
}

pub fn decrypt256(key: [u8; 32], iv: [u8; 16], ciphertext: &[u8]) -> Result<Vec<u8>> {
    let cipher = Aes256CbcDec::new(&key.into(), &iv.into());
    cipher
        .decrypt_padded_vec_mut::<Pkcs7>(ciphertext)
        .map_err(Error::Padding)
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("padding error")]
    Padding(UnpadError),
}

type Result<T> = result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;
    use aes::Aes128;
    use base64::prelude::BASE64_STANDARD;
    use base64::Engine;

    type Aes128CbcEnc = Encryptor<Aes128>;
    type Aes128CbcDec = Decryptor<Aes128>;

    pub fn encrypt128(key: [u8; 16], iv: [u8; 16], msg: &[u8]) -> Vec<u8> {
        let cipher = Aes128CbcEnc::new(&key.into(), &iv.into());
        cipher.encrypt_padded_vec_mut::<Pkcs7>(msg)
    }

    pub fn decrypt128(key: [u8; 16], iv: [u8; 16], ciphertext: &[u8]) -> Result<Vec<u8>> {
        let cipher = Aes128CbcDec::new(&key.into(), &iv.into());
        cipher
            .decrypt_padded_vec_mut::<Pkcs7>(ciphertext)
            .map_err(Error::Padding)
    }

    fn get_plaintext() -> [u8; 34] {
        *b"hello world! this is my plaintext."
    }

    fn get_ciphertext128() -> Vec<u8> {
        hex::decode(
            "c7fe247ef97b21f07cbdd26cb5d346bfd27867cb00d9486723e159978fb9a5f914cfb228a710de4171e396e7b6cf859e"
        ).unwrap()
    }

    #[test]
    fn encrypt128_works() {
        let key = [0x42; 16];
        let iv = [0x24; 16];
        let got = encrypt128(key, iv, &get_plaintext());
        let want = get_ciphertext128();
        assert_eq!(got, want);
    }

    #[test]
    fn decrypt128_works() -> Result<()> {
        let key = [0x42; 16];
        let iv = [0x24; 16];
        let got = decrypt128(key, iv, &get_ciphertext128())?;
        let want = get_plaintext();
        assert_eq!(got, want);
        Ok(())
    }

    fn get_ciphertext256() -> Vec<u8> {
        hex::decode(
            "1718b1dfc1f147fdf82f6ed08445c4512c861b013c808c928851c3c771b5df350620bcec613c8e336963859970e876bf"
        ).unwrap()
    }

    #[test]
    fn encrypt256_works() {
        let key = [0x42; 32];
        let iv = [0x24; 16];
        let got = encrypt256(key, iv, &get_plaintext());
        let want = get_ciphertext256();
        assert_eq!(got, want);
    }

    #[test]
    fn decrypt256_works() -> Result<()> {
        let key = [0x42; 32];
        let iv = [0x24; 16];
        let got = decrypt256(key, iv, &get_ciphertext256())?;
        let want = get_plaintext();
        assert_eq!(got, want);
        Ok(())
    }

    fn get_shared_secret() -> [u8; 32] {
        hex::decode("a2c2394b2e37d7fa70184ec34d1a89a27e3b318312e2534d812be2dc2543a44b")
            .unwrap()
            .try_into()
            .unwrap()
    }

    fn get_shared_ciphertext() -> Vec<u8> {
        BASE64_STANDARD
            .decode("Sttp6Sv7aui5Q3DnJl2Rb7geyKSY+8BFDvAfm/iBievSa5NndvPYBMuMk2fwI9Sq")
            .unwrap()
    }

    fn get_shared_iv() -> [u8; 16] {
        BASE64_STANDARD
            .decode("xbJan2ZwvllmnWlORG7VjA==")
            .unwrap()
            .try_into()
            .unwrap()
    }

    #[test]
    fn encrypt256_using_shared_secret() {
        let got = encrypt256(get_shared_secret(), get_shared_iv(), &get_plaintext());
        let want = get_shared_ciphertext();
        assert_eq!(got, want);
    }

    #[test]
    fn decrypt256_using_shared_secret() -> Result<()> {
        let got = decrypt256(
            get_shared_secret(),
            get_shared_iv(),
            &get_shared_ciphertext(),
        )?;
        let want = get_plaintext();
        assert_eq!(got, want);
        Ok(())
    }
}
