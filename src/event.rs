use crate::time::{self, Seconds};
use crate::{Hex, Kind, Tag};

use secp256k1::hashes::{self, hex, sha256::Hash};
use secp256k1::schnorr::Signature;
use secp256k1::{Message, XOnlyPublicKey, SECP256K1};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io;
use std::str::FromStr;

/// Event is at the heart of nostr
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Event {
    id: Hex,
    pubkey: Hex,
    created_at: Seconds,
    kind: Kind,
    tags: Vec<Tag>,
    content: String,
    sig: Hex,
}

impl Event {
    /// new constructs an event, calculates the id, signs the payload,
    /// and populates the public key deriving it from the secret key.
    pub fn new(
        kind: Kind,
        tags: Vec<Tag>,
        content: String,
        secretkey: &secp256k1::SecretKey,
    ) -> Self {
        let pair = secp256k1::KeyPair::from_secret_key(SECP256K1, secretkey);
        let (pubkey, _) = pair.x_only_public_key();
        let created_at = time::since_epoch();
        let mut event = Self {
            id: "".to_string(),
            pubkey: pubkey.to_string(),
            created_at,
            kind,
            tags,
            content,
            sig: "".to_string(),
        };
        let id = Message::from_slice(event.hash().as_ref()).expect("message must be 32 bytes");
        let sig = pair.sign_schnorr(id);
        event.id = id.to_string();
        event.sig = sig.to_string();
        event
    }

    /// verifies signature matches the id and the pubkey.
    pub fn verify(&self) -> Result<(), Error> {
        if self.hash().to_string() != self.id {
            return Err(Error::HashMismatch);
        }
        let digest = Hash::from_str(&self.id)?;
        let signature = &Signature::from_str(&self.sig)?;
        let message = &Message::from_slice(digest.as_ref())?;
        let pubkey = &XOnlyPublicKey::from_str(&self.pubkey)?;
        SECP256K1.verify_schnorr(signature, message, pubkey)?;
        Ok(())
    }

    /// hashes the event fields.
    fn hash(&self) -> Hash {
        let json = &json!([
            0,
            self.pubkey,
            self.created_at,
            self.kind,
            self.tags,
            self.content
        ]);
        let data = serde_json::to_string(json).expect("unable to serialize json");
        hashes::Hash::hash(data.as_ref())
    }
}

#[derive(Debug)]
pub enum Error {
    HashMismatch,
    Verification(secp256k1::Error),
    Hex(hex::Error),
}

impl From<secp256k1::Error> for Error {
    fn from(err: secp256k1::Error) -> Self {
        Error::Verification(err)
    }
}

impl From<hex::Error> for Error {
    fn from(err: hex::Error) -> Self {
        Error::Hex(err)
    }
}

impl From<Error> for io::Error {
    fn from(err: Error) -> Self {
        match err {
            Error::HashMismatch => io::Error::new(io::ErrorKind::Other, "hash mismatch"),
            Error::Verification(_err) => io::Error::new(io::ErrorKind::Other, "verification error"),
            Error::Hex(_err) => io::Error::new(io::ErrorKind::Other, "hex error"),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use serde_json::{from_str, to_string};

    pub fn get_simple_event() -> Event {
        Event {
            id: "id".into(),
            pubkey: "pubkey".into(),
            created_at: 0,
            kind: 1,
            tags: vec![vec![
                "p".to_string(),
                "profile".to_string(),
                "relays".to_string(),
            ]],
            content: "content".into(),
            sig: "sig".into(),
        }
    }

    pub fn get_json() -> &'static str {
        r#"{"id":"id","pubkey":"pubkey","created_at":0,"kind":1,"tags":[["p","profile","relays"]],"content":"content","sig":"sig"}"#
    }

    #[test]
    fn serialize_works() -> serde_json::Result<()> {
        let event = get_simple_event();
        let got = to_string(&event)?;
        let want = get_json();
        assert_eq!(got, want);
        Ok(())
    }

    #[test]
    fn deserialize_works() -> serde_json::Result<()> {
        let data = get_json();
        let got: Event = from_str(data)?;
        let want = get_simple_event();
        assert_eq!(got, want);
        Ok(())
    }

    fn get_event() -> Event {
        Event{
            id: "6623d3fb9270903631ee00c9683be7065726244518ea3fe334b3b490a8bece20".into(),
            pubkey: "c2e54fc64221e3b58dd960507db72909956cc0aa41019626ca64112984b85c2d".into(),
            created_at: 1675631647,
            kind: 70202,
            tags: vec![],
            content: "test".into(),
            sig: "aaeba9765a6a6a82833fc5593fc3fe70997371a4fbd50afc064e2a50d7c21b2a7910f796ead8a4fcd2f7c592b8603c9cbe4f4756c6650127ba8334782ca53247".into(),
        }
    }

    #[test]
    fn hash_works() {
        let event = get_event();
        let hash = event.hash();
        assert_eq!(hash.to_string(), event.id);
    }

    #[test]
    fn verification_works() -> Result<(), Error> {
        get_event().verify()?;
        Ok(())
    }

    #[test]
    pub fn new_is_idempotent() -> Result<(), Error> {
        let (sk, _) = secp256k1::generate_keypair(&mut secp256k1::rand::thread_rng());
        let event = Event::new(0, vec![], "content".to_string(), &sk);
        println!("{:?}", event);
        event.verify()?;
        Ok(())
    }
}
