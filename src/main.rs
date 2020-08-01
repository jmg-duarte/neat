use clap::Clap;

const VERSION : &'static str= env!("CARGO_PKG_VERSION");
const AUTHORS : &'static str= env!("CARGO_PKG_AUTHORS");

#[derive(Clap)]
#[clap(version=VERSION, author=AUTHORS)]
struct Opts {
    /// The path to the configuration file.
    #[clap(short, long, default_value="neat.toml")]
    config: String,
    /// Target folder to be organized.
    target: String,
}

fn main() {
    let opts = Opts::parse();
}
