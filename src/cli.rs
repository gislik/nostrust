use crate::event::{Event, Kind};
use crate::key::Pair;
use crate::message::MessageRequest;
use crate::request::Request;
use crate::Hex;
use std::io::{Error, ErrorKind, Read, Result, Write};

pub fn io_error(message: &str) -> Error {
    Error::new(ErrorKind::Other, message)
}

pub fn read_event<R: Read>(reader: R) -> Result<Event> {
    let event = serde_json::from_reader(reader)?;
    Ok(event)
}

pub fn verify_event<R: Read>(reader: R) -> Result<()> {
    let event = read_event(reader)?;
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

pub fn event_message_request<R: Read, W: Write>(reader: R, writer: W) -> Result<()> {
    let event = read_event(reader)?;
    let message = MessageRequest::Event(event);
    serde_json::to_writer(writer, &message)?;
    Ok(())
}

pub fn write_request<W: Write>(writer: W, authors: Vec<Hex>, kinds: Vec<u32>) -> Result<()> {
    let mut request = Request::new();
    request.set_authors(authors);
    request.set_kinds(kinds);
    serde_json::to_writer(writer, &request)?;
    Ok(())
}

pub fn read_request<R: Read>(reader: R) -> Result<Request> {
    let request = serde_json::from_reader(reader)?;
    Ok(request)
}

pub fn request_message_request<R: Read, W: Write>(reader: R, writer: W, id: String) -> Result<()> {
    let request = read_request(reader)?;
    let message = MessageRequest::Request(id, request);
    serde_json::to_writer(writer, &message)?;
    Ok(())
}
