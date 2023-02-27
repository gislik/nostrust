pub mod event;
pub mod profile;
pub use bech32::{FromBase32, ToBase32};
use thiserror::Error;

pub const SPECIAL_TYPE: u8 = 0x0;
pub const EVENT_SIZE: u8 = 0x20;
pub const RELAY_TYPE: u8 = 0x1;
pub const PUBKEY_SIZE: u8 = 0x20;

pub fn decode(prefix: &str, data: &str) -> Result<Vec<u8>> {
    let (hrp, data, variant) = bech32::decode(data)?;
    if hrp != prefix {
        return Error::prefix(prefix, hrp);
    }
    if variant != bech32::Variant::Bech32 {
        return Error::variant();
    }
    let data: Vec<u8> = FromBase32::from_base32(&data)?;
    Ok(data)
}

pub fn encode(prefix: &str, data: Vec<u8>) -> Result<String> {
    bech32::encode(prefix, data.to_base32(), bech32::Variant::Bech32).map_err(From::from)
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid prefix (expected {expected:?}, found {found:?})")]
    InvalidPrefix { expected: String, found: String },
    #[error("variant must be bech32")]
    Variant,
    #[error("bech32 encoding error")]
    Bech32(#[from] bech32::Error),
}

impl Error {
    fn prefix<T>(expected: &str, found: String) -> Result<T> {
        Err(Error::InvalidPrefix {
            expected: expected.to_string(),
            found,
        })
    }

    fn variant<T>() -> Result<T> {
        Err(Error::Variant)
    }
}

fn advance_by<I: Iterator>(iter: &mut I, n: usize) -> &mut I {
    iter.nth(n - 1);
    iter
}
