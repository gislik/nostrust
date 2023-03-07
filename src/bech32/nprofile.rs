use std::result;

use crate::bech32::{self, *};
use crate::key::PublicKey;

const PROFILE_PREFIX: &str = "nprofile";

#[derive(Debug, PartialEq)]
pub struct Profile {
    public_key: Option<PublicKey>,
    relays: Vec<String>,
}

impl ToBech32 for Profile {
    fn to_bech32(&self) -> String {
        let mut bytes = vec![SPECIAL_TYPE, PUBKEY_SIZE];
        let bs = self
            .public_key
            .map_or([0; PUBKEY_SIZE as usize], |x| x.serialize());
        bytes.append(&mut bs.as_slice().to_owned());
        for relay in &self.relays {
            let mut bs = relay.as_bytes().to_owned();
            bytes.append(&mut vec![RELAY_TYPE, bs.len() as u8]);
            bytes.append(&mut bs);
        }
        bech32::encode(PROFILE_PREFIX, bytes).expect("encoding nprofile")
    }
}

impl FromBech32 for Profile {
    type Err = bech32::Error;

    fn from_bech32(s: &str) -> Result<Self> {
        let bytes = bech32::decode(PROFILE_PREFIX, s)?;
        let mut iter = bytes.iter();
        let mut profile = Profile {
            public_key: None,
            relays: vec![],
        };
        while let Some(n) = iter.next() {
            match n {
                &SPECIAL_TYPE => {
                    let size = *iter.next().ok_or(Error::MissingLength)? as usize;
                    if size != PUBKEY_SIZE as usize {
                        return Error::invalid_length(PUBKEY_SIZE as usize, size);
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
                &other => return Error::invalid_type(other),
            }
        }
        if iter.len() != 0 {
            return Error::unexpected_data(iter.copied().collect());
        }
        Ok(profile)
    }
}

type Result<T> = result::Result<T, bech32::Error>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::key;

    impl Profile {
        pub fn new(public_key: PublicKey, relays: Vec<String>) -> Self {
            Self {
                public_key: Some(public_key),
                relays,
            }
        }
    }

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
        let got = profile.to_bech32();
        let want = "nprofile1qqsrhuxx8l9ex335q7he0f09aej04zpazpl0ne2cgukyawd24mayt8gpp4mhxue69uhhytnc9e3k7mgpz4mhxue69uhkg6nzv9ejuumpv34kytnrdaksjlyr9p";
        assert_eq!(got, want);
    }

    #[test]
    fn profile_from_nprofile() -> Result<()> {
        let nprofile = "nprofile1qqsrhuxx8l9ex335q7he0f09aej04zpazpl0ne2cgukyawd24mayt8gpp4mhxue69uhhytnc9e3k7mgpz4mhxue69uhkg6nzv9ejuumpv34kytnrdaksjlyr9p";
        let got = Profile::from_bech32(nprofile)?;
        let want = get_profile();
        assert_eq!(got, want);
        Ok(())
    }
}
