pub mod cli;

use anyhow::Result;
use clap::Parser;
use cli::env::*;
use cli::*;
use nostrust::Pair;

fn main() -> Result<()> {
    let pair = var("SECRET_KEY")
        .and_then(|x| Ok(Pair::new(x)?))
        .or_missing(var("NSEC").and_then(|x| Ok(Pair::from_nsec(x)?)))
        .or_missing(var("MNEMONIC").and_then(|x| Ok(Pair::from_mnemonic(x)?)))
        .or_missing(Var::new(Pair::generate()));

    let args = Args::parse();
    handle_args(args, &pair.to_result()?)
}
