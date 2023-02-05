use crate::{Epoch, Hex, Kind, CONTEXT};
use secp256k1::hashes::{self, hex, sha256::Hash};
use secp256k1::schnorr::Signature;
use secp256k1::{Message, XOnlyPublicKey};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::str::FromStr;

/// Event is at the heart of nostr
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Event {
    id: Hex,
    pubkey: Hex,
    created_at: Epoch,
    kind: Kind,
    tags: Vec<String>,
    content: String,
    sig: Hex,
}

impl Event {
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

    /// verifies signature matches the id and the pubkey.
    fn verify(&self) -> Result<(), Error> {
        if self.hash().to_string() != self.id {
            return Err(Error::HashMismatch);
        }
        let digest = Hash::from_str(&self.id).map_err(Error::Hex)?;
        let signature = &Signature::from_str(&self.sig).map_err(Error::Verification)?;
        let message = &Message::from_slice(digest.as_ref()).map_err(Error::Verification)?;
        let pubkey = &XOnlyPublicKey::from_str(&self.pubkey).map_err(Error::Verification)?;
        CONTEXT
            .verify_schnorr(signature, message, pubkey)
            .map_err(Error::Verification)
    }
}

#[derive(Debug)]
enum Error {
    HashMismatch,
    Verification(secp256k1::Error),
    Hex(hex::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{from_str, to_string};

    fn get_simple_event() -> Event {
        Event {
            id: "id".into(),
            pubkey: "pubkey".into(),
            created_at: 0,
            kind: 1,
            tags: vec![],
            content: "content".into(),
            sig: "sig".into(),
        }
    }

    fn get_json() -> &'static str {
        r#"{"id":"id","pubkey":"pubkey","created_at":0,"kind":1,"tags":[],"content":"content","sig":"sig"}"#
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
}
