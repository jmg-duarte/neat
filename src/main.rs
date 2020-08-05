mod neat;

use std::fs;

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
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    let conf = fs::read(&opts.config).expect("something failed while reading the configuration");
    let toml_conf: neat::config::Config =
        toml::from_slice(conf.as_slice()).expect("something went wrong parsing the config");
    neat::run(toml_conf, opts)
}
