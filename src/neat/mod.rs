#![deny(rust_2018_idioms)]

pub mod config;
pub mod error;
pub mod matcher;

use std::fmt::Debug;
use std::fs;
use std::path::{Path, PathBuf};

use crate::neat::config::Mapping;
use crate::neat::error::NeatError;
use crate::Opts;

fn build_file_path(folder: &Path, file: &Path) -> Result<PathBuf, NeatError> {
    let file_name = file.file_name().and_then(|os_str| os_str.to_str());
    match file_name {
        Some(fname) => {
            let mut folder_path_buf = folder.to_path_buf();
            folder_path_buf.push(fname);
            Ok(folder_path_buf)
        }
        None => Err(NeatError::Filename(file.to_path_buf())),
    }
}

fn get_move_op<P, Q>(dry_run: bool, verbose: u8) -> impl Fn(P, Q) -> Result<(), NeatError>
where
    P: AsRef<Path> + Debug,
    Q: AsRef<Path> + Debug,
{
    match (dry_run, verbose) {
        (true, _) => |from, to| {
            println!("moving {:?} to {:?}", from, to);
            Ok(())
        },
        (false, 0) => |from, to| {
            fs::rename(from, to).map_err(|source| error::NeatError::Io(source))?;
            Ok(())
        },
        (false, _) => |from, to| {
            println!("moving {:?} to {:?}", from, to);
            fs::rename(from, to).map_err(|source| error::NeatError::Io(source))?;
            Ok(())
        },
    }
}

pub(crate) fn exec(opts: &Opts, mapping: &Mapping) -> Result<(), NeatError> {
    let matcher = matcher::Matcher::new(
        opts.target.as_str(),
        &mapping.folder,
        &mapping.glob,
        opts.case_sensitive,
    );
    let paths = matcher.run(opts.dry_run, opts.verbose)?;
    let op = get_move_op::<PathBuf, PathBuf>(opts.dry_run, opts.verbose);
    for path in paths {
        let file = path.map_err(|source| error::NeatError::Glob(source))?;
        let to = build_file_path(matcher.to.as_path(), file.as_path())?;
        op(file, to)?;
    }
    Ok(())
}
