use std::{path::PathBuf, process::Command};
use rayon::ThreadPoolBuilder;
use crate::error::TurboGitError;

pub mod error;

pub type TurboGitResult<T> = Result<T, TurboGitError>;

fn main() -> TurboGitResult<()> {
    let cpus = num_cpus::get();
    let pool = ThreadPoolBuilder::new().num_threads(cpus).build()?;
    
    for repo in glob::glob("[!.]*/").unwrap() {
        match repo {
            Ok(path) => {
                pool.install(|| fetch(&path).unwrap())
            }
            Err(e) => println!("{}", e),
        }
    }
    Ok(())
}

fn fetch(repo: &PathBuf) -> TurboGitResult<()> {
    let fetch = Command::new("git").current_dir(repo).arg("fetch").output()?;
    println!("{:?}", fetch);
    Ok(())
}