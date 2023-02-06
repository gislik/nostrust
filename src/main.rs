use clap::{Parser, Subcommand};
use nostrust::cli::*;
use std::io::{stdin, Result};

#[derive(Parser)]
#[command(author, version, about, long_about)]
pub struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Event {
        #[command(subcommand)]
        subcommand: Option<EventCommand>,
    },
}

#[derive(Subcommand)]
pub enum EventCommand {
    Verify,
}

fn main() -> Result<()> {
    let args = Args::parse();
    match args.command {
        Some(Command::Event { subcommand }) => match subcommand {
            Some(EventCommand::Verify) => verify_event(stdin()),
            None => Ok(()),
        },
        None => Ok(()),
    }
}

