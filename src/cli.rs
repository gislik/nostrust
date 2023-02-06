use crate::event::Event;
use std::io::{Read, Result};

pub fn verify_event<R: Read>(reader: R) -> Result<()> {
    let event: Event = serde_json::from_reader(reader)?;
    event.verify()?;
    println!("valid");
    Ok(())
}
