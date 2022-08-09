use rayon::ThreadPoolBuilder;
use crate::{subcommands::fetch, error::TurboGitError};

pub mod error;
pub mod subcommands;

// TurboGitResult type
pub type TurboGitResult<T> = Result<T, TurboGitError>;

fn main() -> TurboGitResult<()> {
    let cpus = num_cpus::get();
    let pool = ThreadPoolBuilder::new().num_threads(cpus).build()?;
    let dir = glob::glob("[!.]*/").unwrap();
    
    for repo in dir {
        let repo = repo?;
        pool.install(|| fetch(&repo).unwrap());
    }
    Ok(())
}