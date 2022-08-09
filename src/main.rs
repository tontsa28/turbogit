use clap::Parser;
use rayon::ThreadPoolBuilder;
use crate::{subcommands::{fetch, pull, Cli, Commands}, error::TurboGitError};

pub mod error;
pub mod subcommands;

// TurboGitResult type
pub type TurboGitResult<T> = Result<T, TurboGitError>;

fn main() -> TurboGitResult<()> {
    let cli = Cli::parse();
    let cpus = num_cpus::get();
    let pool = ThreadPoolBuilder::new().num_threads(cpus).build()?;
    let dir = glob::glob("[!.]*/").unwrap();

    match &cli.command {
        Some(Commands::Fetch) => {
            for repo in dir {
                let repo = repo?;
                pool.install(|| fetch(&repo).unwrap());
            }
        }
        Some(Commands::Pull) => {
            for repo in dir {
                let repo = repo?;
                pool.install(|| pull(&repo).unwrap());
            }
        }
        None => {}
    }
    Ok(())
}