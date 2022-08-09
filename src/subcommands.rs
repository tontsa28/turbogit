use std::{path::PathBuf, process::Command};
use crate::TurboGitResult;

// Fetch subcommand
pub fn fetch(repo: &PathBuf) -> TurboGitResult<()> {
    let fetch = Command::new("git").current_dir(repo).arg("fetch").output()?;

    if fetch.status.success() {
        println!("Successfully fetched {}", repo.display());
    } else {
        println!("Failed to fetch {}", repo.display());
    }
    Ok(())
}

// Pull subcommand
pub fn pull(repo: &PathBuf) -> TurboGitResult<()> {
    let pull = Command::new("git").current_dir(repo).arg("pull").output()?;

    if pull.status.success() {
        println!("Successfully pulled {}", repo.display());
    } else {
        println!("Failed to pull {}", repo.display());
    }
    Ok(())
}