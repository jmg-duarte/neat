#![deny(rust_2018_idioms)]

pub mod config;
pub mod error;

use std::fmt::Debug;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
// use colored::*;
use glob::{glob_with, MatchOptions};

use crate::neat::config::Mapping;
use crate::Opts;

/// Create the folder from `path`, it also creates any missing folders.
/// For instance, the path `/a/b/c` will create the folder `c` as well as folders `a` and `b` if they do not exist.
fn create_folders(path: &Path, dry_run: bool) -> Result<()> {
    if dry_run {
        println!("creating {:?}", path);
        Ok(())
    } else {
        match (path.exists(), path.is_dir()) {
            (false, _) => {
                if let Err(e) = fs::create_dir_all(&path) {
                    Err(anyhow!(error::NeatError::Io(e)))
                } else {
                    Ok(())
                }
            }
            (true, false) => Err(anyhow!(error::NeatError::NamingConflict(String::from(
                path.to_str().unwrap()
            )))),
            (_, _) => Ok(()),
        }
    }
}

fn get_match_options(case_sensitive: bool) -> MatchOptions {
    let mut match_opts = MatchOptions::new();
    match_opts.case_sensitive = case_sensitive;
    match_opts
}

fn build_glob(folder: &str, glob: &str) -> String {
    let mut result = folder.to_owned();
    if !result.ends_with("/") && !glob.starts_with("/") {
        result.push('/');
    }
    result.push_str(glob);
    result
}

fn build_file_path(folder: &Path, file: &Path) -> PathBuf {
    let file_name = file
        .file_name()
        .and_then(|os_str| os_str.to_str())
        .expect("Failed to read the filename");
    let mut folder_path_buf = folder.to_path_buf();
    folder_path_buf.push(file_name);
    folder_path_buf
}

fn exec(opts: &Opts, mapping: &Mapping) -> Result<()> {
    let target = opts.target.as_str();
    let case_sensitive = opts.case_sensitive;
    let match_opts = get_match_options(case_sensitive);
    let mut folder_path = PathBuf::from(&target);
    folder_path.push(&mapping.folder);
    create_folders(&folder_path, opts.dry_run)?;
    // println!("{}: {:?}", "folder_path".blue(), folder_path);
    let target_glob = build_glob(&target, &mapping.glob);
    // println!("{}: {}", "glob".blue(), target_glob);
    let paths =
        glob_with(&target_glob, match_opts).map_err(|source| error::NeatError::Pattern(source))?;
    let op = get_move_op::<PathBuf, PathBuf>(opts.dry_run);
    for path in paths {
        let from = path.map_err(|source| error::NeatError::Glob(source))?;
        let to = build_file_path(folder_path.as_path(), from.as_path());
        op(from, to)?;
    }
    Ok(())
}

fn get_move_op<P, Q>(dry_run: bool) -> impl Fn(P, Q) -> Result<()>
where
    P: AsRef<Path> + Debug,
    Q: AsRef<Path> + Debug,
{
    if dry_run {
        |from, to| {
            println!("moving {:?} to {:?}", from, to);
            Ok(())
        }
    } else {
        |from, to| {
            fs::rename(from, to).map_err(|source| error::NeatError::Io(source))?;
            Ok(())
        }
    }
}

pub(crate) fn run(conf: config::Config, opts: crate::Opts) -> Result<()> {
    let mappings = conf.mapping;
    for m in mappings {
        exec(&opts, &m)?;
    }
    Ok(())
}
