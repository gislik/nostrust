use std::str::FromStr;

use secp256k1 as ec;
use secp256k1::schnorr;

#[derive(PartialEq, Debug)]
pub struct Signature(schnorr::Signature);

impl ToString for Signature {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl FromStr for Signature {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sig = schnorr::Signature::from_str(s)?;
        Ok(Signature(sig))
    }
}

impl From<schnorr::Signature> for Signature {
    fn from(sig: schnorr::Signature) -> Self {
        Self(sig)
    }
}

#[derive(Debug)]
pub enum Error {
    Signature(ec::Error),
}

impl From<ec::Error> for Error {
    fn from(err: ec::Error) -> Self {
        Error::Signature(err)
    }
}
