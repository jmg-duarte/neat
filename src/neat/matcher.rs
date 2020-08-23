use crate::neat::error::NeatError;

use std::fs;
use std::path::{Path, PathBuf};

use colored::*;
use glob::{glob_with, MatchOptions, Paths};

pub struct Matcher {
    from: PathBuf,
    pub to: PathBuf,
    glob: Glob,
}

impl Matcher {
    pub fn new(from: &str, to: &str, glob: &str, case_sensitive: bool) -> Self {
        Matcher {
            from: PathBuf::from(&from),
            to: {
                let mut p = PathBuf::from(&from);
                p.push(&to);
                p
            },
            glob: Glob::new(from, glob, case_sensitive),
        }
    }

    pub fn run(&self, dry_run: bool, verbose: u8) -> Result<Paths, NeatError> {
        if verbose > 1 {
            println!("{}: {:?}", "folder_path".blue(), self.from);
            println!("{}: {}", "glob".blue(), self.glob.glob);
        }
        create_folders(&self.to, dry_run, verbose)?;
        let paths = self.glob.glob()?;
        Ok(paths)
    }
}

struct Glob {
    glob: String,
    options: MatchOptions,
}

impl Glob {
    fn new(base_dir: &str, glob: &str, case_sensitive: bool) -> Self {
        Glob {
            glob: build_glob(&base_dir, &glob),
            options: get_match_options(case_sensitive),
        }
    }

    fn glob(&self) -> Result<Paths, glob::PatternError> {
        glob_with(&self.glob, self.options)
    }
}

fn build_glob(folder: &str, glob: &str) -> String {
    let mut result = folder.to_owned();
    if !result.ends_with("/") && !glob.starts_with("/") {
        result.push('/');
    }
    result.push_str(glob);
    result
}

fn get_match_options(case_sensitive: bool) -> MatchOptions {
    let mut match_opts = MatchOptions::new();
    match_opts.case_sensitive = case_sensitive;
    match_opts
}

/// Create the folder from `path`, it also creates any missing folders.
/// For instance, the path `/a/b/c` will create the folder `c` as well as folders `a` and `b` if they do not exist.
fn create_folders(path: &Path, dry_run: bool, verbose: u8) -> Result<(), NeatError> {
    if dry_run {
        println!("create folder {:?}", path);
    } else {
        if verbose > 0 {
            println!("create folder {:?}", path);
        }
        fs::create_dir_all(&path).map_err(|e| NeatError::NamingConflict {
            file: path.to_path_buf(),
            err: e,
        })?;
    }
    Ok(())
}
