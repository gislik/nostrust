use std::io::ErrorKind;
use std::str::FromStr;
use std::{char, io, vec};

use crate::key::{self, Pair, PublicKey};
use crate::signature::{self, Signature};
use crate::time::{self, Seconds};
use crate::Hex;
use secp256k1::hashes::{self, hex, hex::FromHex, sha256::Hash};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// METADATA is defined by [NIP-01](https://github.com/nostr-protocol/nips/blob/master/01.md).
const METADATA: Kind = 0;
/// TEXT is defined by [NIP-01](https://github.com/nostr-protocol/nips/blob/master/01.md).
const TEXT: Kind = 1;
/// RECOMMEND_RELAY is defined by [NIP-01](https://github.com/nostr-protocol/nips/blob/master/01.md).
const RECOMMEND_RELAY: Kind = 2;
/// RECOMMEND_RELAY is defined by [NIP-02](https://github.com/nostr-protocol/nips/blob/master/02.md).
const CONTACT_LIST: Kind = 3;

/// E is defined by [NIP-01](https://github.com/nostr-protocol/nips/blob/master/01.md).
const E: char = 'e';
/// P is defined by [NIP-01](https://github.com/nostr-protocol/nips/blob/master/01.md).
const P: char = 'p';

/// Event is at the heart of nostr. Defined in
/// [NIP-01](https://github.com/nostr-protocol/nips/blob/master/01.md).
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
    /// Defined in [NIP-01](https://github.com/nostr-protocol/nips/blob/master/01.md).
    pub fn new(kind: Kind, tags: Vec<Tag>, content: &str, pair: &Pair) -> Self {
        let pubkey = pair.public_key();
        let created_at = time::since_epoch();
        let mut event = Self {
            id: "".to_string(),
            pubkey: pubkey.to_string(),
            created_at,
            kind,
            tags,
            content: content.to_string(),
            sig: "".to_string(),
        };
        let id = event.hash();
        let sig = pair.sign(id).unwrap(); // hash is always valid
        event.id = id.to_string();
        event.sig = sig.to_string();
        event
    }

    /// Constructs a new event which sets the metadata of the public key.
    /// Defined in [NIP-01](https://github.com/nostr-protocol/nips/blob/master/01.md).
    pub fn set_metadata(name: &str, about: &str, picture: &str, pair: &Pair) -> Self {
        let content = json!({
            "name": name,
            "about": about,
            "picture": picture,
        });
        Event::new(METADATA, vec![], &content.to_string(), pair)
    }

    /// Constructs a new text note.
    /// Defined in [NIP-01](https://github.com/nostr-protocol/nips/blob/master/01.md).
    pub fn text_note(content: &str, pair: &Pair) -> Self {
        Event::new(TEXT, vec![], content, pair)
    }

    /// Constructs a recommend relay note.
    /// Defined in [NIP-01](https://github.com/nostr-protocol/nips/blob/master/01.md).
    pub fn recommend_relay(relay: &str, pair: &Pair) -> Self {
        Event::new(RECOMMEND_RELAY, vec![], relay, pair)
    }

    /// Constructs a new contact list.
    /// Defined in [NIP-02](https://github.com/nostr-protocol/nips/blob/master/02.md).
    pub fn contact_list(contacts: Vec<Contact>, pair: &Pair) -> Self {
        let tags = contacts
            .iter()
            .map(|c| Tag::profile(c.key.to_owned(), &c.relay, &c.petname))
            .collect();
        Event::new(CONTACT_LIST, tags, "", pair)
    }

    /// Sets the tags of an event.
    pub fn set_tags(&mut self, tags: &Vec<Tag>) -> &mut Self {
        self.tags = tags.to_owned();
        self
    }

    /// verifies signature matches the id and the pubkey.
    pub fn verify(&self) -> Result<()> {
        if self.hash().to_string() != self.id {
            return Err(Error::HashMismatch);
        }
        let sig = Signature::from_str(&self.sig)?;
        let data = Vec::<u8>::from_hex(&self.id)?;
        let pk = PublicKey::from_str(&self.pubkey)?;
        Pair::from(&pk).verify(&sig, &data, &pk)?;
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

/// Kind denotes the event kind
pub type Kind = u32;

/// Tag denotes the event tag
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Tag(Vec<String>);

impl Tag {
    pub fn event(id: Hex, relay: &str) -> Self {
        Tag(vec![E.to_string(), id, relay.to_string()])
    }

    pub fn profile(key: Hex, relay: &str, petname: &str) -> Self {
        Tag(vec![
            P.to_string(),
            key.to_string(),
            relay.to_string(),
            petname.to_string(),
        ])
    }
}

pub struct Contact {
    key: Hex,
    relay: String,
    petname: String,
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
#[error("event error")]
pub enum Error {
    HashMismatch,
    Signature(signature::Error),
    Verification(key::Error),
    Hex(hex::Error),
}

impl From<key::Error> for Error {
    fn from(err: key::Error) -> Self {
        Error::Verification(err)
    }
}

impl From<signature::Error> for Error {
    fn from(err: signature::Error) -> Self {
        Error::Signature(err)
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
            Error::HashMismatch => io_error("hash mismatch"),
            Error::Verification(_err) => io_error("verification error"),
            Error::Signature(_err) => io_error("signature error"),
            Error::Hex(_err) => io_error("hex error"),
        }
    }
}

pub fn io_error(message: &str) -> io::Error {
    io::Error::new(ErrorKind::Other, message)
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
            tags: vec![Tag::profile("profile".to_string(), "relays", "petname")],
            content: "content".into(),
            sig: "sig".into(),
        }
    }

    pub fn get_json() -> &'static str {
        r#"{"id":"id","pubkey":"pubkey","created_at":0,"kind":1,"tags":[["p","profile","relays","petname"]],"content":"content","sig":"sig"}"#
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
    fn verification_works() -> Result<()> {
        get_event().verify()?;
        Ok(())
    }

    #[test]
    pub fn new_is_idempotent() -> Result<()> {
        let pair = Pair::generate();
        let event = Event::new(0, vec![], "content", &pair);
        println!("{:?}", event);
        event.verify()?;
        Ok(())
    }

    fn get_ots_json() -> &'static str {
        r#"{"id":"id","pubkey":"pubkey","created_at":0,"kind":1,"tags":[["p","profile","relays","petname"]],"content":"content","sig":"sig","ots":"ots"}"#
    }

    #[test]
    fn deserialize_with_ots_works() -> serde_json::Result<()> {
        let data = get_ots_json();
        let got: Event = from_str(data)?;
        let want = get_simple_event();
        assert_eq!(got, want);
        Ok(())
    }
}
