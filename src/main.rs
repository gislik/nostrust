use clap::{Parser, Subcommand};
use nostrust::cli::*;
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
}

fn main() -> Result<()> {
    let args = Args::parse();
    match args.command {
        Command::Event { subcommand } => match subcommand {
            EventCommand::Verify => verify_event(stdin()),
        },
    }
}

