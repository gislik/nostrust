pub mod env;

use std::io::{stdin, stdout, Read, Write};

use anyhow::Result;
use clap::{Parser, Subcommand};
use nostrust::{Event, Hex, Kind, MessageRequest, Pair, Request};

#[derive(Parser)]
#[command(author, version, about, long_about)]
pub struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Verify and generate events
    Event {
        #[command(subcommand)]
        subcommand: EventCommand,
    },
    /// Generate requests
    Request {
        #[arg(short, long)]
        ids: Vec<Hex>,
        #[arg(short, long)]
        authors: Vec<Hex>,
        #[arg(short, long)]
        kinds: Vec<u32>,
        #[arg(short, long)]
        e: Vec<Hex>,
        #[arg(short, long)]
        p: Vec<Hex>,
        #[arg(short, long)]
        since: Option<u32>,
        #[arg(short, long)]
        until: Option<u32>,
        #[arg(short, long)]
        limit: Option<u16>,
    },
    /// Generate message requests
    MessageRequest {
        #[command(subcommand)]
        subcommand: MessageRequestCommand,
    },
    /// Print key
    Key,
}

#[derive(Subcommand)]
pub enum EventCommand {
    /// Verifies an event on stdin
    Verify,
    /// Output a new event to stdout
    Generate {
        #[arg(short, long)]
        kind: Kind,
        #[arg(short, long)]
        subject: Option<String>,
        content: String,
    },
    /// Output a new set metadata event to stdout
    SetMetadata {
        name: String,
        about: String,
        picture: String,
    },
    /// Output a new text note to stdout
    TextNote { content: String },
    /// Output a new recommend relay to stdout
    RecommendRelay { relay: String },
}

// #[derive(Subcommand)]
// pub enum RequestCommand {}

#[derive(Subcommand)]
pub enum MessageRequestCommand {
    Event,
    Request { id: String },
}

pub fn handle_args(args: Args, pair: &Pair) -> Result<()> {
    match args.command {
        Command::Event { subcommand } => match subcommand {
            EventCommand::Verify => verify_event(stdin())?,
            EventCommand::Generate {
                kind,
                content,
                subject,
            } => generate_event(kind, subject, &content)?,
            EventCommand::SetMetadata {
                name,
                about,
                picture,
            } => set_metadata_event(&name, &about, &picture)?,
            EventCommand::TextNote { content } => text_note_event(&content)?,
            EventCommand::RecommendRelay { relay } => recommend_relay_event(&relay)?,
        },
        Command::Request {
            ids,
            authors,
            kinds,
            e,
            p,
            since,
            until,
            limit,
        } => write_request(stdout(), ids, authors, kinds, e, p, since, until, limit)?,
        Command::MessageRequest { subcommand } => match subcommand {
            MessageRequestCommand::Event => event_message_request(stdin(), stdout())?,
            MessageRequestCommand::Request { id } => {
                request_message_request(stdin(), stdout(), id)?
            }
        },
        Command::Key => print_key(&mut stdout(), pair)?,
    };
    Ok(())
}

pub fn read_event<R: Read>(reader: R) -> Result<Event> {
    let event = serde_json::from_reader(reader)?;
    Ok(event)
}

pub fn verify_event<R: Read>(reader: R) -> Result<()> {
    let event = read_event(reader)?;
    event.verify()?;
    println!("Event is valid ✅");
    Ok(())
}

pub fn generate_event(kind: Kind, subject: Option<String>, content: &str) -> Result<()> {
    let pair = Pair::generate();
    let mut event = Event::new(kind, vec![], content, &pair);
    event.set_subject(subject);
    serde_json::to_writer(stdout(), &event)?;
    Ok(())
}

pub fn set_metadata_event(name: &str, about: &str, picture: &str) -> Result<()> {
    let pair = Pair::generate();
    let event = Event::set_metadata(name, about, picture, &pair);
    serde_json::to_writer(stdout(), &event)?;
    Ok(())
}

pub fn text_note_event(content: &str) -> Result<()> {
    let pair = Pair::generate();
    let event = Event::text_note(content, &pair);
    serde_json::to_writer(stdout(), &event)?;
    Ok(())
}

pub fn recommend_relay_event(relay: &str) -> Result<()> {
    let pair = Pair::generate();
    let event = Event::recommend_relay(relay, &pair);
    serde_json::to_writer(stdout(), &event)?;
    Ok(())
}

pub fn event_message_request<R: Read, W: Write>(reader: R, writer: W) -> Result<()> {
    let event = read_event(reader)?;
    let message = MessageRequest::Event(event);
    serde_json::to_writer(writer, &message)?;
    Ok(())
}

pub fn write_request<W: Write>(
    writer: W,
    ids: Vec<Hex>,
    authors: Vec<Hex>,
    kinds: Vec<u32>,
    e: Vec<Hex>,
    p: Vec<Hex>,
    since: Option<u32>,
    until: Option<u32>,
    limit: Option<u16>,
) -> Result<()> {
    let mut request = Request::new();
    request
        .set_ids(ids)
        .set_authors(authors)
        .set_kinds(kinds)
        .set_events(e)
        .set_profiles(p);
    if let Some(since) = since {
        request.set_since(since);
    }
    if let Some(until) = until {
        request.set_until(until);
    }
    if let Some(limit) = limit {
        request.set_limit(limit);
    }
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

pub fn print_key<W: Write>(writer: &mut W, pair: &Pair) -> Result<()> {
    writer.write_all(pair.secret_key().unwrap().display_secret_as_nsec().as_ref())?;
    Ok(())
}
