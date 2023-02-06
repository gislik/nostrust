use crate::event::Event;
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
