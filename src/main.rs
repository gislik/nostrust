use clap::{Parser, Subcommand};
use nostrust::cli::*;
use nostrust::event::Kind;
use std::io::{stdin, Result};

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
    }
}
