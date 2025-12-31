use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
    #[arg(short, long, value_name = "PATH")]
    pub dir: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Parse Markdown files in the given directory
    Parse,
    /// For debugging, we expose an interface to get the AST of some given file
    Ast {
        path: PathBuf,
        #[arg(short, long)]
        json: bool
    }
}
