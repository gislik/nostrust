use crate::event::{Event, Kind};
use crate::key::Pair;
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

pub fn set_metadata_event(name: &str, about: &str, picture: &str) -> Result<()> {
    let pair = Pair::generate();
    let event = Event::set_metadata(name, about, picture, &pair);
    serde_json::to_writer(std::io::stdout(), &event)?;
    Ok(())
}

pub fn text_note_event(content: &str) -> Result<()> {
    let pair = Pair::generate();
    let event = Event::text_note(content, &pair);
    serde_json::to_writer(std::io::stdout(), &event)?;
    Ok(())
}

pub fn recommend_relay_event(relay: &str) -> Result<()> {
    let pair = Pair::generate();
    let event = Event::recommend_relay(relay, &pair);
    serde_json::to_writer(std::io::stdout(), &event)?;
    Ok(())
}
