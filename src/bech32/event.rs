use super::{advance_by, EVENT_SIZE, SPECIAL_TYPE};
use crate::n::{self, RELAY_TYPE};
use crate::Hex;
use std::str::Utf8Error;
use thiserror::Error;

const EVENT_PREFIX: &str = "nevent";

#[derive(Debug, PartialEq)]
pub struct Event {
    id: Hex,
    relays: Vec<String>,
}

impl Event {
    pub fn from_nevent(data: &str) -> Result<Self> {
        let data = n::decode(EVENT_PREFIX, data)?;
        let mut iter = data.iter();
        let mut event = Event {
            id: "".to_string(),
            relays: vec![],
        };
        while let Some(n) = iter.next() {
            match n {
                &SPECIAL_TYPE => {
                    let size = *iter.next().ok_or(Error::MissingLength)? as usize;
                    let iter2 = iter.clone().take(size);
                    let data: Vec<u8> = iter2.copied().collect();
                    advance_by(&mut iter, size);
                    event.id = std::str::from_utf8(&data)?.to_string();
                }
                &RELAY_TYPE => {
                    let size = *iter.next().ok_or(Error::MissingLength)? as usize;
                    let iter2 = iter.clone().take(size);
                    let data: Vec<u8> = iter2.copied().collect();
                    let str: &str = std::str::from_utf8(&data)?;
                    advance_by(&mut iter, size);
                    event.relays.push(str.to_string());
                }
                other => return Error::invalid_type(*other),
            }
        }
        if iter.len() != 0 {
            return Error::unexpected_data(iter.copied().collect());
        }
        Ok(event)
    }

    pub fn to_nevent(&self) -> String {
        let mut data = vec![SPECIAL_TYPE, EVENT_SIZE];
        data.append(&mut self.id.as_bytes().to_owned());
        for relay in &self.relays {
            let mut bs = relay.as_bytes().to_owned();
            data.append(&mut vec![RELAY_TYPE, bs.len() as u8]);
            data.append(&mut bs);
        }
        n::encode(EVENT_PREFIX, data).expect("encoding nevent")
    }
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
#[error("nevent error")]
pub enum Error {
    #[error("invalid type (found {found})")]
    InvalidType {
        found: u8,
    },
    #[error("length is missing")]
    MissingLength,
    #[error("unexpected data (found {found:?})")]
    UnexpectedData {
        found: Vec<u8>,
    },
    Bech32(#[from] n::Error),
    #[error("utf8 error")]
    Utf8Error(#[from] Utf8Error),
}

impl Error {
    fn invalid_type<T>(found: u8) -> Result<T> {
        Err(Error::InvalidType { found })
    }

    fn unexpected_data<T>(found: Vec<u8>) -> Result<T> {
        Err(Error::UnexpectedData { found })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_simple_event() -> Event {
        Event {
            id: "6623d3fb9270903631ee00c9683be706".to_string(),
            relays: vec![],
        }
    }

    #[test]
    fn simple_event_to_nevent() {
        let event = get_simple_event();
        let got = event.to_nevent();
        let want = "nevent1qqsrvd3jxdjrxenz8yerwvpexqenvve3v4jnqvrr8ymrsvmzv5mnqdscemr6j";
        assert_eq!(got, want);
    }

    #[test]
    fn simple_event_from_nevent() -> Result<()> {
        let nevent = "nevent1qqsrvd3jxdjrxenz8yerwvpexqenvve3v4jnqvrr8ymrsvmzv5mnqdscemr6j";
        let got = Event::from_nevent(nevent)?;
        let want = get_simple_event();
        assert_eq!(got, want);
        Ok(())
    }

    fn get_event() -> Event {
        Event {
            id: "6623d3fb9270903631ee00c9683be706".to_string(),
            relays: vec![
                "wss://localhost:4000".to_string(),
                "wss://localhost:4001".to_string(),
            ],
        }
    }

    #[test]
    fn event_to_nevent() {
        let event = get_event();
        let got = event.to_nevent();
        let want = "nevent1qqsrvd3jxdjrxenz8yerwvpexqenvve3v4jnqvrr8ymrsvmzv5mnqdspz3mhxue69uhkcmmrv9kxsmmnwsargvpsxqq3gamnwvaz7tmvda3kzmrgdaehgw35xqcrzzl46w7";
        assert_eq!(got, want);
    }

    #[test]
    fn event_from_nevent() -> Result<()> {
        let nevent = "nevent1qqsrvd3jxdjrxenz8yerwvpexqenvve3v4jnqvrr8ymrsvmzv5mnqdspz3mhxue69uhkcmmrv9kxsmmnwsargvpsxqq3gamnwvaz7tmvda3kzmrgdaehgw35xqcrzzl46w7";
        let got = Event::from_nevent(nevent)?;
        let want = get_event();
        assert_eq!(got, want);
        Ok(())
    }
}
