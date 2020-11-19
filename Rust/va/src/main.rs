mod data_store;
extern crate clap;
use clap::{App, load_yaml, crate_version};

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yaml");
    // let _matches = App::from_yaml(yaml).version(crate_version!()).get_matches();
    // let mut s = data_store::Store::load().unwrap();
    let mut note = data_store::Note::new();
    note.priority(50);
    let mut dc = data_store::Todo::new();
    dc.title("newsdcsd".to_string());
    let mut dc1 = data_store::Todo::new();
    dc.tag("newsdcdscsd".to_string());
    // s.add_note(note);
    // s.save()
    // let mut store = data_store::Store::new();
    // store.add(note);
    // store.show();
    // store.add(dc);
    // store.add(dc1);
    // store.show();

    println!("{}",std::any::type_name::<data_store::Note>());
    // Same as previous examples...
}