mod data_store;
extern crate clap;
use clap::{App, load_yaml, crate_version};

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yaml");
    // let _matches = App::from(yaml).version(crate_version!()).get_matches();
    // let mut s = data_store::Store::load().unwrap();
    let mut note = data_store::Note::new();
    note.priority(50);
    let mut dc = data_store::Todo::new();
    dc.title("Todo".to_string());
    let mut dc1 = data_store::Todo::new();
    let a = data_store::Note::new();
    dc1.tag("Tag test".to_string());
    // s.add_note(note);
    // s.save()
    let mut store = data_store::Store::new();
    store.add(note);
    store.add(a);
    store.show();
    store.add(dc);
    store.add(dc1);
    store.remove(&(Box::new(data_store::Note::new()) as Box<dyn data_store::Storable>), data_store::Note::new());
    store.show();
    store.save().unwrap();

    println!("{}",std::any::type_name::<data_store::Note>());
    // Same as previous examples...
}