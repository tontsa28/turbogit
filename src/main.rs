use clap::Parser;
use crate::{commands::{fetch::fetch, pull::pull, Cli, Commands}, error::TurboGitError};

mod error;
mod commands;

// TurboGitResult type
pub type TurboGitResult<T> = Result<T, TurboGitError>;

fn main() -> TurboGitResult<()> {
    let cli = Cli::parse();

    // Check if subcommands were given
    match &cli.command {
        Some(Commands::Fetch) => fetch()?,
        Some(Commands::Pull) => pull()?,
        None => {}
    }
    Ok(())
}