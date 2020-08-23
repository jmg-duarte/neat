use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum NeatError {
    #[error("File exists with the name {file}")]
    NamingConflict {
        file: PathBuf,
        #[source]
        err: std::io::Error,
    },

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Pattern(#[from] glob::PatternError),

    #[error(transparent)]
    Glob(#[from] glob::GlobError),
}
