use thiserror::Error;

#[derive(Debug, Error)]
pub enum TurboGitError {
    #[error("{}", 0)]
    IOError(#[from] std::io::Error),
    #[error("{}", 0)]
    ThreadPoolBuildError(#[from] rayon::ThreadPoolBuildError),
}