use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::Path;

use clap::Clap;
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
}

#[derive(Deserialize)]
struct Config {
    /// A map which folders to globs.
    /// Files matching the globs will be moved to the folders.
    mappings: Vec<HashMap<String, String>>,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.mappings)
    }
}

fn execute_mapping(target: &String, mappings: &HashMap<String, String>) {
    let folder = mappings.get("folder").unwrap();
    let glob_str = mappings.get("glob").unwrap();
    let folder_path = Path::new(folder);
    match (folder_path.exists(), folder_path.is_dir()) {
        (false, _) => {
            fs::create_dir_all(&folder_path).expect("something went wrong creating the folder")
        }
        (true, false) => panic!("file exists with folder name {}", folder),
        _ => {}
    }
    let mut target_glob = target.clone();
    if !target_glob.ends_with("/") {
        target_glob.push('/');
    }
    target_glob.push_str(glob_str);
    println!("{}", target_glob);
    let mut match_opts = MatchOptions::new();
    match_opts.case_sensitive = false; // TODO make optional
    for entry in glob_with(&target_glob, match_opts).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                println!("{:?}", path.display());
                let mut target_file = folder.clone();
                if !target_file.ends_with("/") {
                    target_file.push('/');
                }
                target_file.push_str(path.to_str().expect("Invalid path"));
                fs::rename(path, Path::new(&target_file)).expect("Failed while copying the files");
            }
            Err(e) => println!("{:?}", e),
        }
    }
}

fn main() {
    let opts = Opts::parse();
    let conf = fs::read(opts.config).expect("something failed while reading the configuration");
    let toml_conf: Config =
        toml::from_slice(conf.as_slice()).expect("something went wrong parsing the config");

    println!("{}", toml_conf);

    let mappings = toml_conf.mappings;
    for (idx, m) in mappings.iter().enumerate() {
        match (m.get("folder"), m.get("glob")) {
            (Some(_), Some(_)) => execute_mapping(&opts.target, &m),
            (None, Some(_)) => panic!(
                "mapping ({}) does not contain the required fields 'folder'",
                idx
            ),
            (Some(_), None) => panic!(
                "mapping ({}) does not contain the required fields 'glob'",
                idx
            ),
            _ => panic!(
                "mapping ({}) does not contain the required fields 'folder' and 'glob'",
                idx
            ),
        }
    }
}
