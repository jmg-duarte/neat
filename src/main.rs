use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use clap::Clap;
use colored::*;
use glob::{glob_with, MatchOptions};
use serde::Deserialize;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

#[derive(Clap)]
#[clap(version=VERSION, author=AUTHORS)]
struct Opts {
    /// The path to the configuration file.
    #[clap(short, long, default_value = "neat.toml")]
    config: String,
    /// Target folder to be organized.
    target: String,
    /// Wether the globs are case sensitive or not.
    #[clap(short = "i")]
    case_sensitive: bool,
    #[clap(short = "n", long)]
    dry_run: bool,
}

#[derive(Deserialize)]
struct Config {
    /// A map which folders to globs.
    /// Files matching the globs will be moved to the folders.
    mapping: Vec<HashMap<String, String>>,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.mapping)
    }
}

fn create_folders(path: &Path) -> Result<(), String> {
    println!("creating {:?}", path);
    match (path.exists(), path.is_dir()) {
        (false, _) => fs::create_dir_all(&path).map_err(|err| err.to_string()),
        (true, false) => Err(format!(
            "file exists with folder name {}",
            path.to_str().unwrap()
        )),
        _ => Ok(()),
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

fn execute_mapping(opts: &Opts, mappings: &HashMap<String, String>) -> Result<(), String> {
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

fn run(conf: Config, opts: Opts) -> Result<(), String> {
    let mappings = conf.mapping;
    for (idx, m) in mappings.iter().enumerate() {
        match (m.get("folder"), m.get("glob")) {
            (Some(_), Some(_)) => execute_mapping(&opts, &m),
            (None, Some(_)) => Err(format!(
                "Mapping [{}] does not contain the required field \"folder\"",
                idx
            )),
            (Some(_), None) => Err(format!(
                "Mapping [{}] does not contain the required field \"glob\"",
                idx
            )),
            _ => Err(format!(
                "Mapping [{}] does not contain the required fields \"folder\" and \"glob\"",
                idx
            )),
        }?;
    }
    Ok(())
}

fn main() -> Result<(), String> {
    let opts = Opts::parse();
    let conf = fs::read(&opts.config).expect("something failed while reading the configuration");
    let toml_conf: Config =
        toml::from_slice(conf.as_slice()).expect("something went wrong parsing the config");
    run(toml_conf, opts)
}
