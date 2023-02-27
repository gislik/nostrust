use super::advance_by;
use crate::key::{self, PublicKey};
use crate::n::{self, PUBKEY_SIZE, RELAY_TYPE, SPECIAL_TYPE};
use thiserror::Error;

const PROFILE_PREFIX: &str = "nprofile";

#[derive(Debug, PartialEq)]
pub struct Profile {
    public_key: Option<PublicKey>,
    relays: Vec<String>,
}

impl Profile {
    pub fn new(public_key: PublicKey, relays: Vec<String>) -> Self {
        Self {
            public_key: Some(public_key),
            relays,
        }
    }

    pub fn from_nprofile(nprofile: &str) -> Result<Profile> {
        let data = n::decode(PROFILE_PREFIX, nprofile)?;
        let mut iter = data.iter();
        let mut profile = Profile {
            public_key: None,
            relays: vec![],
        };
        while let Some(n) = iter.next() {
            match n {
                &SPECIAL_TYPE => {
                    let size = *iter.next().ok_or(Error::MissingLength)? as usize;
                    if size != PUBKEY_SIZE as usize {
                        return Error::invalid_length(32, size);
                    }
                    let iter2 = &mut iter.clone().copied().take(size);
                    let public_key = PublicKey::try_from(iter2.collect::<Vec<u8>>().as_ref())?;
                    advance_by(&mut iter, size);
                    profile.public_key = Some(public_key);
                }
                &RELAY_TYPE => {
                    let size = *iter.next().ok_or(Error::MissingLength)? as usize;
                    let iter2 = &mut iter.clone().copied().take(size);
                    let data: Vec<u8> = iter2.collect();
                    let str: &str = std::str::from_utf8(&data)?;
                    advance_by(&mut iter, size);
                    profile.relays.push(str.to_string());
                }
                &other => return Error::invalid_type(RELAY_TYPE, other),
            }
        }
        if iter.len() != 0 {
            return Error::unexpected_data(iter.copied().collect());
        }
        Ok(profile)
    }

    pub fn to_nprofile(&self) -> String {
        let mut output = vec![SPECIAL_TYPE, PUBKEY_SIZE];
        let bs = self.public_key.map_or([0; 32], |x| x.serialize());
        output.append(&mut bs.as_slice().to_owned());
        for relay in &self.relays {
            let mut bs = relay.as_bytes().to_owned();
            output.append(&mut vec![RELAY_TYPE, bs.len() as u8]);
            output.append(&mut bs);
        }
        n::encode(PROFILE_PREFIX, output).expect("encoding nprofile")
    }
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid type (expected {expected}, found {found})")]
    InvalidType { expected: u8, found: u8 },
    #[error("length is missing")]
    MissingLength,
    #[error("invalid length (expected {expected}, found {found})")]
    InvalidLength { expected: usize, found: usize },
    #[error("utf8 error")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("bech32 encoding error")]
    Bech32(#[from] n::Error),
    #[error("key error")]
    Key(#[from] key::Error),
    #[error("unexpected data (found {found:?})")]
    UnexpectedData { found: Vec<u8> },
}

impl Error {
    fn invalid_type<T>(expected: u8, found: u8) -> Result<T> {
        Err(Error::InvalidType { expected, found })
    }

    fn invalid_length<T>(expected: usize, found: usize) -> Result<T> {
        Err(Error::InvalidLength { expected, found })
    }

    fn unexpected_data<T>(found: Vec<u8>) -> Result<T> {
        Err(Error::UnexpectedData { found })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::key;

    fn get_profile() -> Profile {
        let pk = key::tests::get_public_key();
        let relays = vec![
            "wss://r.x.com".to_string(),
            "wss://djbas.sadkb.com".to_string(),
        ];
        Profile::new(pk, relays)
    }

    #[test]
    fn profile_to_nprofile() {
        let profile = get_profile();
        let got = profile.to_nprofile();
        let want = "nprofile1qqsrhuxx8l9ex335q7he0f09aej04zpazpl0ne2cgukyawd24mayt8gpp4mhxue69uhhytnc9e3k7mgpz4mhxue69uhkg6nzv9ejuumpv34kytnrdaksjlyr9p";
        assert_eq!(got, want);
    }

    #[test]
    fn profile_from_nprofile() -> Result<()> {
        let nprofile = "nprofile1qqsrhuxx8l9ex335q7he0f09aej04zpazpl0ne2cgukyawd24mayt8gpp4mhxue69uhhytnc9e3k7mgpz4mhxue69uhkg6nzv9ejuumpv34kytnrdaksjlyr9p";
        let got = Profile::from_nprofile(nprofile)?;
        let want = get_profile();
        assert_eq!(got, want);
        Ok(())
    }
}
