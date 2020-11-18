mod data_store;
extern crate clap;
use clap::{App, load_yaml, crate_version};

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yaml");
    let _matches = App::from_yaml(yaml).version(crate_version!()).get_matches();

    // Same as previous examples...
}