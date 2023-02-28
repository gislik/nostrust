use crate::bech32;
use crate::bech32::nsec::SECRET_PREFIX;
use crate::signature::Signature;
use secp256k1 as ec;
use secp256k1::{schnorr, SECP256K1};
use std::result;
use std::str::FromStr;
use thiserror::Error;

/// Keypair for the secp256k1 elliptic curve. Defined in
/// [NIP-01](https://github.com/nostr-protocol/nips/blob/master/01.md).
pub struct Pair {
    /// The secret key of the key pair. If the secret key doesn't exists
    /// the pair cannot be used to sign messages.
    secret_key: Option<SecretKey>,
    /// The public key of the key pair.
    public_key: PublicKey,
}

impl Pair {
    /// Generates a new SECP256k1 key pair.
    pub fn generate() -> Self {
        let (sk, pk) = ec::generate_keypair(&mut ec::rand::thread_rng());
        let secret_key = Some(SecretKey(sk));
        let (xpk, _) = pk.x_only_public_key();
        let public_key = PublicKey(xpk);
        Self {
            secret_key,
            public_key,
        }
    }

    /// Signs the data and produces a signature.
    pub fn sign<T>(&self, data: T) -> Result<Signature>
    where
        T: AsRef<[u8]>,
    {
        match self.secret_key {
            Some(sk) => {
                let msg = ec::Message::from_slice(data.as_ref())?;
                let keypair = &ec::KeyPair::from_secret_key(SECP256K1, &sk.0);
                let sig = ec::KeyPair::sign_schnorr(keypair, msg);
                Ok(Signature::from(sig))
            }
            None => Err(Error::Signature(
                "no secret key in the key pair".to_string(),
            )),
        }
    }

    /// Verifies a signature and data against a public key.
    pub fn verify<T>(&self, sig: &Signature, data: T, pk: &PublicKey) -> Result<()>
    where
        T: AsRef<[u8]>,
    {
        let signature = &schnorr::Signature::from_str(sig.to_string().as_str())?;
        let message = &ec::Message::from_slice(data.as_ref())?;
        let pubkey = &pk.0;
        SECP256K1.verify_schnorr(signature, message, pubkey)?;
        Ok(())
    }

    /// Returns the secret key of the key pair, if it exists.
    pub fn secret_key(&self) -> Option<&SecretKey> {
        self.secret_key.as_ref()
    }

    /// Returns the public key of the key pair.
    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }
}

impl From<&SecretKey> for Pair {
    fn from(sk: &SecretKey) -> Self {
        let (xpk, _) = sk.0.x_only_public_key(SECP256K1);
        Self {
            secret_key: Some(sk.to_owned()),
            public_key: PublicKey(xpk),
        }
    }
}

impl From<&PublicKey> for Pair {
    fn from(pk: &PublicKey) -> Self {
        Self {
            secret_key: None,
            public_key: pk.to_owned(),
        }
    }
}

/// Secret key
#[derive(Clone, Copy)]
pub struct SecretKey(ec::SecretKey);

impl SecretKey {
    /// Returns the bech32 encoded secret key. Defined in
    /// [NIP-19](https://github.com/nostr-protocol/nips/blob/master/19.md)
    pub fn display_secret_as_nsec(&self) -> String {
        bech32::encode(SECRET_PREFIX, self.0.secret_bytes().into()).unwrap() // never results in an error
    }

    /// Returns the hex encoded secret key
    pub fn display_secret(&self) -> String {
        format!("{}", self.0.display_secret())
    }
}

impl FromStr for SecretKey {
    type Err = Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        let sk = ec::SecretKey::from_str(value)?;
        Ok(SecretKey(sk))
    }
}

impl TryFrom<&[u8]> for SecretKey {
    type Error = Error;

    fn try_from(value: &[u8]) -> result::Result<Self, Self::Error> {
        let sk = ec::SecretKey::from_slice(value)?;
        Ok(SecretKey(sk))
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PublicKey(pub(crate) ec::XOnlyPublicKey);

impl PublicKey {
    pub fn serialize(&self) -> [u8; 32] {
        self.0.serialize()
    }
}

impl FromStr for PublicKey {
    type Err = Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        let xpk = ec::XOnlyPublicKey::from_str(&value)?;
        Ok(PublicKey(xpk))
    }
}

impl TryFrom<&[u8]> for PublicKey {
    type Error = Error;

    fn try_from(value: &[u8]) -> result::Result<Self, Self::Error> {
        let xpk = ec::XOnlyPublicKey::from_slice(value)?;
        Ok(PublicKey(xpk))
    }
}

impl ToString for PublicKey {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("key")]
    Key(#[from] ec::Error),
    // #[error("bech32")]
    // Bech32(#[from] bech32::Error),
    #[error("prefix")]
    Prefix(String),
    #[error("variant")]
    Variant(String),
    #[error("signature")]
    Signature(String),
}

#[cfg(test)]
pub mod tests {
    use super::*;

    fn get_secret_key() -> SecretKey {
        SecretKey::from_str("0f1429676edf1ff8e5ca8202c8741cb695fc3ce24ec3adc0fcf234116f08f849")
            .unwrap()
    }

    #[test]
    fn secret_key_matches() -> Result<()> {
        let got = get_secret_key().display_secret();
        let want = "0f1429676edf1ff8e5ca8202c8741cb695fc3ce24ec3adc0fcf234116f08f849";
        assert_eq!(got, want);
        Ok(())
    }

    #[test]
    fn display_secret_as_nsec() -> Result<()> {
        let got = get_secret_key().display_secret_as_nsec();
        let want = "nsec1pu2zjemwmu0l3ew2sgpvsaquk62lc08zfmp6ms8u7g6pzmcglpysymcg0m";
        assert_eq!(got, want);
        Ok(())
    }

    #[test]
    fn verification_works() -> Result<()> {
        let raw = "3bf0c63fcb93463407af97a5e5ee64fa883d107ef9e558472c4eb9aaaefa459d";
        let sk = SecretKey::from_str(raw)?;
        let pair = Pair::from(&sk);
        let pk = pair.public_key();
        let data = [0x1; 32];
        let sig = Signature::from_str("e235a72aaaa17cb4101d9b67d196a2aa0618cfea19f7a4884a2aea138585c7498b99697bf9b4d5fff4a15883062fd0b2408f44250fccf73cd76b6ce3ce1ac420").unwrap();
        pair.verify(&sig, &data, &pk)?;
        Ok(())
    }

    pub fn get_public_key() -> PublicKey {
        let raw = "3bf0c63fcb93463407af97a5e5ee64fa883d107ef9e558472c4eb9aaaefa459d";
        PublicKey::from_str(raw).unwrap()
    }

    #[test]
    fn public_key_matches() -> Result<()> {
        let got = get_public_key().to_string();
        let want = "3bf0c63fcb93463407af97a5e5ee64fa883d107ef9e558472c4eb9aaaefa459d";
        assert_eq!(got, want);
        Ok(())
    }
}
