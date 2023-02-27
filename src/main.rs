use clap::{Parser, Subcommand};
use nostrust::cli::*;
use nostrust::Kind;
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
    /// Generate a new event on stdout
    Generate {
        #[arg(short, long)]
        kind: Kind,
        content: String,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();
    match args.command {
        Command::Event { subcommand } => match subcommand {
            EventCommand::Verify => verify_event(stdin()),
            EventCommand::Generate { kind, ref content } => generate_event(kind, content),
        },
    }
}
