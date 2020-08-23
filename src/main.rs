mod neat;

use std::fs;

use anyhow::{Context, Result};
use clap::Clap;

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
    #[clap(short, long, parse(from_occurrences))]
    verbose: u8,
}

fn main() -> Result<()> {
    let opts = Opts::parse();
    let conf =
        fs::read(&opts.config).context("An error occurred while reading the configuration file")?;
    let toml_conf: neat::config::Config = toml::from_slice(conf.as_slice())
        .context("An error occurred while parsing the configuration file")?;
    run(toml_conf, opts)
}

fn run(conf: neat::config::Config, opts: crate::Opts) -> Result<()> {
    let mappings = conf.mapping;
    for m in mappings {
        neat::exec(&opts, &m)?;
    }
    Ok(())
}
