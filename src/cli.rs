use crate::{event::Event, key::Pair, Kind};
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

pub fn generate_event(kind: Kind, content: &str) -> Result<()> {
    let pair = Pair::generate();
    let event = Event::new(kind, vec![], content, &pair);
    serde_json::to_writer(std::io::stdout(), &event)?;
    Ok(())
}
