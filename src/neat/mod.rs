#![deny(rust_2018_idioms)]

pub mod config;
pub mod error;

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::fmt::Debug;

use anyhow::{anyhow, Result};
use colored::*;
use glob::{glob_with, MatchOptions};

use crate::neat::config::Mapping;
use crate::Opts;

/// Create the folder from `path`, it also creates any missing folders.
/// For instance, the path `/a/b/c` will create the folder `c` as well as folders `a` and `b` if they do not exist.
fn create_folders(path: &Path) -> anyhow::Result<()> {
    println!("creating {:?}", path);
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

fn execute_mapping(opts: &crate::Opts, mappings: &HashMap<String, String>) -> anyhow::Result<()> {
    let target = opts.target.to_owned();
    let case_sensitive = opts.case_sensitive;
    let match_opts = get_match_options(case_sensitive);

    let folder = mappings.get("folder").unwrap();
    let glob_str = mappings.get("glob").unwrap();
    let mut folder_path = PathBuf::from(&target);
    folder_path.push(folder);

    if opts.dry_run {
        println!("{} :: {:?}", "create folder".green(), folder_path);
    } else {
        create_folders(folder_path.as_path())?;
    }

    let target_glob = build_glob(&target, &glob_str);
    // println!("{}", target_glob);

    for entry in glob_with(&target_glob, match_opts).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                if !folder_path.is_dir() && !opts.dry_run {
                    panic!("folder {:?} is not dir", folder)
                }
                let fp = build_file_path(folder_path.as_path(), path.as_path());
                if opts.dry_run {
                    println!("{} :: {:?} -> {:?}", "move file".green(), path, fp);
                } else {
                    fs::rename(path, fp).expect("Failed while moving the files");
                }
            }
            Err(e) => println!("{:?}", e),
        };
    }
    Ok(())
}

fn exec(opts: &Opts, mapping: &Mapping) -> Result<()> {
    let target = opts.target.as_str();
    let case_sensitive = opts.case_sensitive;
    let match_opts = get_match_options(case_sensitive);
    let mut folder_path = PathBuf::from(&target);
    folder_path.push(&mapping.folder);
    println!("{}: {:?}", "folder_path".blue(), folder_path);
    let target_glob = build_glob(&target, &mapping.glob);
    println!("{}: {}", "glob".blue(), target_glob);
    let paths = glob_with(&target_glob, match_opts)
        .map_err(|source| error::NeatError::Pattern(source))?;
    let op = get_move_op::<PathBuf, PathBuf>(opts.dry_run);
    for path in paths {
        let p = path.map_err(|source| error::NeatError::Glob(source))?;
        let fp = build_file_path(folder_path.as_path(), p.as_path());
        op(p, fp)?;
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
    for (idx, m) in mappings.iter().enumerate() {
        exec(&opts, m)?;
    }
    // for (idx, m) in mappings.iter().enumerate() {
    //     match (m.get("folder"), m.get("glob")) {
    //         (Some(_), Some(_)) => execute_mapping(&opts, &m),
    //         (None, Some(_)) => Err(anyhow!(error::NeatError::MissingFields {
    //             idx: idx,
    //             fields: vec!["folder"]
    //         })),
    //         (Some(_), None) => Err(anyhow!(error::NeatError::MissingFields {
    //             idx: idx,
    //             fields: vec!["glob"]
    //         })),
    //         _ => Err(anyhow!(error::NeatError::MissingFields {
    //             idx: idx,
    //             fields: vec!["folder", "glob"]
    //         })),
    //     }?;
    // }
    Ok(())
}
