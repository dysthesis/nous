use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
    #[arg(short, long, value_name = "PATH")]
    dir: Option<PathBuf>
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Parse Markdown files in the given directory
    Parse,
}
