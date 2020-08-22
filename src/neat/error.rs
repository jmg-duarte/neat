use thiserror::Error;

#[derive(Debug, Error)]
pub enum NeatError<'a> {
    #[error("Mapping #{idx:?} is missing fields: {fields:?}")]
    MissingFields { idx: usize, fields: Vec<&'a str> },

    #[error("File exists with the name {0}")]
    NamingConflict(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Pattern(#[from] glob::PatternError),

    #[error(transparent)]
    Glob(#[from] glob::GlobError),
}
