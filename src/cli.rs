use crate::event::Event;
use std::io::{Error, ErrorKind, Read, Result};

pub fn verify_event<R: Read>(reader: R) -> Result<()> {
    let event: Event = serde_json::from_reader(reader)?;
    event
        .verify()
        .map_err(|_err| Error::new(ErrorKind::Other, "damn"))?;
    println!("valid");
    Ok(())
}
