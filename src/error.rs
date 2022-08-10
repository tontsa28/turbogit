use thiserror::Error;

#[derive(Debug, Error)]
pub enum TurboGitError {
    #[error("{}", 0)]
    IOError(#[from] std::io::Error),
    #[error("{}", 0)]
    GlobError(#[from] glob::GlobError),
}