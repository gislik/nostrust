pub mod bech32;
pub mod cli;
pub mod event;
pub mod key;
pub mod message;
pub mod request;
pub mod signature;
pub mod time;

use crate::bech32 as n;
use thiserror::Error;

pub type Hex = String;
pub type Kind = u32;
pub type Tag = Vec<String>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("profile error")]
    ProfileError(#[from] n::profile::Error),
}
