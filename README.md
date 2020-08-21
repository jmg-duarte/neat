# neat
**neat** is a CLI tool for file organization.
It is currently in its early stages and functionality is severely limited,
any suggestions are welcome, whether related to functionalities or the code itself.

## Quick History
**neat** was born out of a mix of necessity, laziness and curiosity.
My camera outputs three formats (when including video) and I wanted to divide them all into separate folders *automagically*.
At the same time, I wanted to dive in Rust and it was the perfect excuse.

## Goals
**neat** aims to be simple to use, providing both a simple interface and a configuration format.

## Usage
At its core **neat** requires one ingredient to work, a configuration,
the configuration maps the files (which are matched by globs) to the target folders.

### Configuration Example
In the example bellow we see that `mapping` is an array of dictionaries, 
the motivation behind this choice is such that multiple mappings are required to exist (hence the array) and mappings are required to be flexible (so that they can be extended in the future).

```toml
[[mapping]]
folder = "JPG"
glob = "./**/*.jpg"
```

### Running Example
When running **neat**, if no configuration file is explicitly passed in (with the `-c` flag) then **neat** assumes the configuration to be present in the current directory, this file should be named `neat.toml`.
**neat** is then run against the target folder.

```
neat target_folder/
```

### Dry Run

**neat** supports dry runs (nothing is modified and changes are written to the terminal).
To perform a dry run use the `-n` or `--dry-run` flags.

### Help

For more information use `neat -h`, which outputs the following:

```
neat 0.1.0
José Duarte <jmg.duarte@campus.fct.unl.pt>

USAGE:
    neat [FLAGS] [OPTIONS] <target>

ARGS:
    <target>    Target folder to be organized

FLAGS:
    -i               Wether the globs are case sensitive or not
    -n, --dry-run    
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <config>    The path to the configuration file [default: neat.toml]
```