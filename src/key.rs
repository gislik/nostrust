use std::result;
use std::str::FromStr;

use crate::bech32;
use crate::bech32::nsec::SECRET_PREFIX;
use crate::encryption;
use crate::mnemonic;
use crate::mnemonic::Mnemonic;
use crate::signature::Signature;
use secp256k1 as ec;
use secp256k1::schnorr;
use secp256k1::SECP256K1 as curve;
use thiserror::Error;

const KEY_SIZE: usize = 32;

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

    /// Creates a new pair from a mnemonic.
    /// Defined in [NIP-01](https://github.com/nostr-protocol/nips/blob/master/06.md).
    pub fn from_mnemonic(s: &str) -> Result<Self> {
        let mnemonic = Mnemonic::new(s)?;
        Pair::try_from(&mnemonic)
    }

    pub fn new_shared_secret(ours: &SecretKey, theirs: &PublicKey) -> Self {
        let pk = theirs.0.public_key(ec::Parity::Even); // parity is not important
        let sk = ours.0;
        let secret = ec::ecdh::shared_secret_point(&pk, &sk);
        let shared_sk = SecretKey::try_from(&secret[0..KEY_SIZE]).unwrap();
        Pair::from(&shared_sk)
    }

    /// Signs the data and produces a signature.
    pub fn sign<T>(&self, data: T) -> Result<Signature>
    where
        T: AsRef<[u8]>,
    {
        match self.secret_key {
            Some(sk) => {
                let msg = ec::Message::from_slice(data.as_ref())?;
                let keypair = &ec::KeyPair::from_secret_key(curve, &sk.0);
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
        curve.verify_schnorr(signature, message, pubkey)?;
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
        let (xpk, _) = sk.0.x_only_public_key(curve);
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

impl TryFrom<&Mnemonic> for Pair {
    type Error = Error;

    fn try_from(mnemonic: &Mnemonic) -> result::Result<Self, Self::Error> {
        let bytes = mnemonic.to_bytes();
        let sk = SecretKey::try_from(&bytes[..])?;
        let pair = Pair::from(&sk);
        Ok(pair)
    }
}

/// Secret key
#[derive(Clone, Copy)]
pub struct SecretKey(ec::SecretKey);

impl SecretKey {
    /// Returns the ciphertext of the plaintext using AES-256-CBC.
    /// [NIP-04](https://github.com/nostr-protocol/nips/blob/master/04.md)
    pub fn encrypt<T>(&self, plaintext: T, iv: [u8; 16]) -> Vec<u8>
    where
        T: AsRef<[u8]>,
    {
        let key = self.0.secret_bytes();
        encryption::encrypt256(key, iv, plaintext.as_ref())
    }

    /// Returns the plain text of the ciphertext using AES-256-CBC.
    pub fn decrypt<T>(&self, ciphertext: T, iv: [u8; 16]) -> Result<Vec<u8>>
    where
        T: AsRef<[u8]>,
    {
        let key = self.0.secret_bytes();
        let ciphertext = encryption::decrypt256(key, iv, ciphertext.as_ref())?;
        Ok(ciphertext)
    }

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
    pub fn serialize(&self) -> [u8; KEY_SIZE] {
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
    #[error("prefix")]
    Prefix(String),
    #[error("variant")]
    Variant(String),
    #[error("signature")]
    Signature(String),
    #[error("encryption")]
    Encryption(#[from] encryption::Error),
    #[error("mnemonic")]
    Mnemonic(#[from] mnemonic::Error),
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::bech32::ToBech32;

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

    fn get_shared_secret() -> Pair {
        let our_secret_key =
            SecretKey::from_str("86b4ecc7994aec6de588b1472540613de5199fc0ed06a0fc463d33ce62aa66e6")
                .unwrap();
        let their_public_key =
            PublicKey::from_str("0cc0cf586ebed5d568315b585089c84b320b0c3a7f37ab9ba9d45803407fbb9c")
                .unwrap();
        Pair::new_shared_secret(&our_secret_key, &their_public_key)
    }

    #[test]
    fn shared_secret_matches() -> Result<()> {
        let pair = get_shared_secret();
        let got = pair.secret_key().unwrap().display_secret();
        let want = "a2c2394b2e37d7fa70184ec34d1a89a27e3b318312e2534d812be2dc2543a44b";
        assert_eq!(got, want);
        Ok(())
    }

    #[test]
    fn from_mnemonic_works() -> Result<()> {
        let s = crate::mnemonic::tests::get_mnemonic_str();
        let pair = Pair::from_mnemonic(s)?;
        let got = pair.public_key().to_bech32();
        let want = "npub1gw5zyqa9yj2rrq5u683y9sfdpv49hmgfkw37hupgvf5vrtdmr60sspjdzz";
        assert_eq!(got, want);
        Ok(())
    }
}
