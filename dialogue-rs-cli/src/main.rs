#![deny(
    missing_docs,
    rustdoc::missing_crate_level_docs,
    unreachable_pub,
    rust_2018_idioms
)]

//! A terminal-based frontend for dialogue-rs

mod commands;

use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help(true))]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

impl Cli {
    fn path(&self) -> Option<&Path> {
        self.command.as_ref().and_then(|c| c.path())
    }
}

#[derive(Subcommand, Debug)]
#[non_exhaustive]
enum Commands {
    /// Check the syntax of a script
    Check {
        /// The path to the script to check
        path: PathBuf,
    },
    /// Run a script
    Run {
        /// The path to the script to check
        path: PathBuf,
    },
}

impl Commands {
    fn path(&self) -> Option<&Path> {
        match self {
            Commands::Check { path } => Some(path.as_path()),
            Commands::Run { path } => Some(path.as_path()),
        }
    }
}

fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    let path = cli.path();
    let script = path
        .map(std::fs::read_to_string)
        .map(|it| it.map(|script_str| dialogue_rs::Script::parse(&script_str)));

    match cli.command {
        Some(Commands::Check { .. }) => {
            commands::check(&script.expect("path is required, clap will validate this")??)
        }
        Some(Commands::Run { .. }) => {
            commands::run(&script.expect("path is required, clap will validate this")??)
        }
        _ => {
            // clap will print the help message for us
            Ok(())
        }
    }
}
