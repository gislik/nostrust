use std::io::{stdin, stdout, Result};

use clap::{Parser, Subcommand};
use nostrust::cli::*;
use nostrust::event::Kind;
use nostrust::Hex;

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

fn main() -> Result<()> {
    let args = Args::parse();
    match args.command {
        Command::Event { subcommand } => match subcommand {
            EventCommand::Verify => verify_event(stdin()),
            EventCommand::Generate { kind, content } => generate_event(kind, &content),
            EventCommand::SetMetadata {
                name,
                about,
                picture,
            } => set_metadata_event(&name, &about, &picture),
            EventCommand::TextNote { content } => text_note_event(&content),
            EventCommand::RecommendRelay { relay } => recommend_relay_event(&relay),
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
        } => write_request(stdout(), ids, authors, kinds, e, p, since, until, limit),
        Command::MessageRequest { subcommand } => match subcommand {
            MessageRequestCommand::Event => event_message_request(stdin(), stdout()),
            MessageRequestCommand::Request { id } => request_message_request(stdin(), stdout(), id),
        },
        Command::Key => print_key(&mut stdout()),
    }
}
