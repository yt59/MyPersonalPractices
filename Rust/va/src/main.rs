mod data_store;
extern crate clap;
use data_store::{Note, Todo, Store};
use clap::{App, load_yaml, crate_version};

fn main() {
    // let yaml = load_yaml!("cli.yaml");
    // let matches = App::from(yaml).version(crate_version!()).get_matches();
    // if let Some(store_file) = matches.value_of("import"){
    //     println!("get {}", store_file);
    // }
    let s =Store::load();
    s.unwrap().show();
}