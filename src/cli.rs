use crate::{event::Event, Kind};
use std::io::{Error, ErrorKind, Read, Result};

pub fn io_error(message: &str) -> Error {
    Error::new(ErrorKind::Other, message)
}

pub fn verify_event<R: Read>(reader: R) -> Result<()> {
    let event: Event = serde_json::from_reader(reader)?;
    event.verify()?;
    println!("valid");
    Ok(())
}

pub fn generate_event(kind: Kind, content: String) -> Result<()> {
    let (sk, _) = &secp256k1::generate_keypair(&mut secp256k1::rand::thread_rng());
    let event = Event::new(kind, vec![], content, sk);
    serde_json::to_writer(std::io::stdout(), &event)?;
    Ok(())
}
