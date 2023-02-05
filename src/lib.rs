use secp256k1::{Secp256k1, VerifyOnly};

#[macro_use]
extern crate lazy_static;

mod event;
mod message;
mod request;

lazy_static! {
    static ref CONTEXT: Secp256k1<VerifyOnly> = Secp256k1::verification_only();
}

type Hex = String;
type Kind = u32;
type Epoch = u32;
