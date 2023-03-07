use std::result;

use bip32::{Language, XPrv};
use secp256k1 as ec;

const DERIVATION_PATH: &str = "m/44'/1237'/0'/0/0";

pub struct Mnemonic(bip32::Mnemonic);

impl Mnemonic {
    pub fn new(phrase: &str) -> Result<Self> {
        let m = bip32::Mnemonic::new(phrase, Language::English)?;
        Ok(Mnemonic(m))
    }

    pub fn random() -> Self {
        let m = bip32::Mnemonic::random(ec::rand::thread_rng(), Language::English);
        Mnemonic(m)
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        let seed = self.0.to_seed("");
        let child_xprv = XPrv::derive_from_path(&seed, &DERIVATION_PATH.parse().unwrap()).unwrap();
        child_xprv.private_key().to_bytes().into()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("BIP-32 error")]
    Bip32(#[from] bip32::Error),
}

type Result<T> = result::Result<T, Error>;

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn get_mnemonic_str() -> &'static str {
        "mule south voice warrior garage broken body dolphin rent pool liar father cost fire prosper scale aspect rack bomb essay ancient vault zero cherry"
    }

    #[test]
    fn to_bytes_matches() -> Result<()> {
        let mnemonic = Mnemonic::new(get_mnemonic_str())?;
        let got = mnemonic.to_bytes();
        let want = [
            5, 206, 100, 89, 138, 186, 221, 182, 89, 221, 77, 156, 165, 9, 130, 97, 253, 62, 156,
            151, 211, 61, 44, 75, 1, 67, 84, 219, 224, 41, 255, 7,
        ];
        assert_eq!(got, want);
        Ok(())
    }
}
