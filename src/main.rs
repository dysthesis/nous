use clap::Parser;
use color_eyre::eyre::Result;
use crate::cli::Cli;

mod cli;

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse(); 
    println!("Received CLI argument: {cli:?}");
    Ok(())
}
