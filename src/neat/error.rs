use thiserror::Error;

#[derive(Debug, Error)]
pub enum NeatError {
    #[error("File exists with the name {0}")]
    NamingConflict(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Pattern(#[from] glob::PatternError),

    #[error(transparent)]
    Glob(#[from] glob::GlobError),
}
