use std::{env, fs::read_to_string};

use clap::Parser;
use color_eyre::{Section, eyre::Result};
use walkdir::WalkDir;
use crate::cli::Cli;

mod cli;

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse(); 
    println!("Received CLI argument: {cli:?}");

    let root_dir = cli.dir
        // Default to the current working directory
        .map_or_else(env::current_dir, Ok)
        .with_note(|| "While getting the current working directory as a fallback, as no `dir` was not provided")?;

    println!("Using directory: {root_dir:?}");

    let mut walkdir_errors = Vec::new();
    let mut io_errors = Vec::new();

    let files: Vec<(walkdir::DirEntry, String)> = WalkDir::new(root_dir)
        .into_iter()
        .filter_map(|e| match e {
            Ok(val) => Some(val),
            Err(err) => {
                walkdir_errors.push(err);
                None
            },
        })
        // Filter for Markdown files
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
        .filter_map(|e| match read_to_string(e.path()) {
            Ok(content) => Some((e, content)),
            Err(err) => {
                io_errors.push(err);
                None
            },
        }).collect();

    files.iter().for_each(|(file, content)| {
        println!("{:?}", file.path());
        println!("{content}");
    });

    Ok(())
}
