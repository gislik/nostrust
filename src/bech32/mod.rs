pub mod nevent;
pub mod nprofile;
pub mod npub;
pub mod nsec;

use std::{result, str::Utf8Error};

use crate::key;
pub use bech32::{FromBase32, ToBase32};
use thiserror::Error;

pub const SPECIAL_TYPE: u8 = 0x0;
pub const EVENT_SIZE: u8 = 0x20;
pub const RELAY_TYPE: u8 = 0x1;
pub const PUBKEY_SIZE: u8 = 0x20;

pub trait ToBech32 {
    /// Encodes the public key to its bech32 encoding. Defined in
    /// [NIP-19](https://github.com/nostr-protocol/nips/blob/master/19.md)
    fn to_bech32(&self) -> String;
}

pub trait FromBech32: Sized {
    type Err;

    /// Tries to parse a public key from its bech32 encoding. Defined in
    /// [NIP-19](https://github.com/nostr-protocol/nips/blob/master/19.md)
    fn from_bech32(s: &str) -> result::Result<Self, Self::Err>;
}

/// Encode a byte slice to bech32 encoded string with a prefix.
pub fn encode(prefix: &str, data: Vec<u8>) -> Result<String> {
    Ok(bech32::encode(
        prefix,
        data.to_base32(),
        bech32::Variant::Bech32,
    )?)
}

/// Docode a string slice with a prefix to byte slice.
pub fn decode(prefix: &str, data: &str) -> Result<Vec<u8>> {
    let (hrp, data, variant) = bech32::decode(data)?;
    if hrp != prefix {
        return Error::invalid_prefix(prefix, hrp);
    }
    if variant != bech32::Variant::Bech32 {
        return Error::variant();
    }
    let data: Vec<u8> = FromBase32::from_base32(&data)?;
    Ok(data)
}

type Result<T> = result::Result<T, Error>;

#[derive(Debug, Error)]
#[error("bech32 error")]
pub enum Error {
    #[error("invalid type (found {found})")]
    InvalidType {
        found: u8,
    },
    UnexpectedData {
        found: Vec<u8>,
    },
    #[error("invalid prefix (expected {expected:?}, found {found:?})")]
    InvalidPrefix {
        expected: String,
        found: String,
    },
    #[error("invalid length (expected {expected}, found {found})")]
    InvalidLength {
        expected: usize,
        found: usize,
    },
    #[error("utf8 error")]
    Utf8Error(#[from] Utf8Error),
    #[error("variant must be bech32")]
    Variant,
    #[error("bech32 encoding error")]
    Bech32(#[from] bech32::Error),
    #[error("length is missing")]
    MissingLength,
    #[error("key error")]
    Key(#[from] key::Error),
}

impl Error {
    fn invalid_type<T>(found: u8) -> Result<T> {
        Err(Error::InvalidType { found })
    }

    fn unexpected_data<T>(found: Vec<u8>) -> Result<T> {
        Err(Error::UnexpectedData { found })
    }

    fn invalid_prefix<T>(expected: &str, found: String) -> Result<T> {
        Err(Error::InvalidPrefix {
            expected: expected.to_string(),
            found,
        })
    }

    fn invalid_length<T>(expected: usize, found: usize) -> Result<T> {
        Err(Error::InvalidLength { expected, found })
    }

    fn variant<T>() -> Result<T> {
        Err(Error::Variant)
    }
}

fn advance_by<I: Iterator>(iter: &mut I, n: usize) -> &mut I {
    iter.nth(n - 1);
    iter
}
