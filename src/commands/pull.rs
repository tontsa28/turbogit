use std::process::{Command, Stdio};
use crate::TurboGitResult;

// Pull subcommand
pub fn pull() -> TurboGitResult<()> {
    // Check for directories in the current working directory
    let dir = glob::glob("[!.]*/").unwrap();
    let mut children = Vec::new();

    // Spawn a new git process for every repository
    for repo in dir {
        let repo = repo?;
        let child = Command::new("git")
            .current_dir(repo)
            .arg("pull")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        children.push(child);
    }

    // Wait for all git processes to complete
    for mut child in children {
        let status = child.wait()?;

        // Check if processes succeeded
        if status.success() {
            println!("Successfully pulled");
        } else {
            println!("Failed to pull");
        }
    }
    Ok(())
}