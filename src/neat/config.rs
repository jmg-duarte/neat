use std::fmt;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    /// A map which folders to globs.
    /// Files matching the globs will be moved to the folders.
    pub mapping: Vec<Mapping>,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.mapping)
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct Mapping {
    pub glob: String,
    pub folder: String
}