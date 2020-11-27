mod data_store;
extern crate clap;
use std::io::Read;

use chrono::DateTime;
use clap::{crate_version, load_yaml, App};
use data_store::{Note, Storable, Store, Todo};

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).version(crate_version!()).get_matches();
    if let Some(store_file) = matches.value_of("import") {
        println!("get {}", store_file);
    }
    let mut store = Store::load().unwrap();
    let save = |s: &mut Store| match s.save() {
        Ok(_) => {
            println!("Changes saved!");
        }
        Err(e) => {
            eprintln!("following error caused changes didn't save: {}", e);
        }
    };
    let are_you_sure = |about: &str| -> bool {
        println!("Are you sure {}? [y/N]", about);
        let mut buffer = [0u8; 1];
        match std::io::stdin().lock().read_exact(&mut buffer) {
            Ok(_) => {
                let ch = *buffer.first().unwrap();
                ch == b'y' || ch == b'Y'
            }
            Err(_) => false,
        }
    };
    match matches.subcommand() {
        Some(("note", note)) => match note.subcommand() {
            Some(("import", import)) => {
                let file = import.value_of("JSONFILE").unwrap();
                match std::fs::canonicalize(file) {
                    Ok(path) => {
                        println!("parsing {:?}", path);
                        match serde_json::from_str::<Vec<Note>>(&Store::load_file(path).unwrap()) {
                            Ok(vec) => store.add_vec::<Note>(vec),
                            Err(e) => {
                                eprintln!("{} is not a json array of notes! -> {:?}", file, e)
                            }
                        };
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
            }
            Some(("create", create)) => {
                let mut n = Note::new();
                n.subject(create.value_of("subject").unwrap().to_string())
                    .explanation(create.value_of("explanation").unwrap().to_string())
                    .conclusion(create.value_of("conclusion").unwrap().to_string());
                if let Some(on) = create.value_of("on") {
                    match DateTime::parse_from_rfc3339(on) {
                        Ok(d) => {
                            n.on(d.timestamp());
                        }
                        Err(e) => {
                            eprintln!("can't parse date-time -> {}", e);
                        }
                    }
                }
                if let Some(priority) = create.value_of("priority") {
                    match priority.parse::<u8>() {
                        Ok(p) => {
                            n.priority(p);
                        }
                        Err(e) => {
                            eprintln!("{} priority must be in range [0-255]", e);
                        }
                    }
                }
                store.add::<Note>(n);
                save(&mut store);
            }
            Some(("update", update)) => {
                match store.find::<Note>(update.value_of("pattern").unwrap()) {
                    Some(res) => {
                        if res.len() > 0 {
                            let old = serde_json::from_str::<Note>(&res.first().unwrap()).unwrap();
                            let mut new =
                                serde_json::from_str::<Note>(&res.first().unwrap()).unwrap();
                            if let Some(subject) = update.value_of("subject") {
                                new.subject(subject.to_string());
                            }
                            if let Some(explanation) = update.value_of("explanation") {
                                new.explanation(explanation.to_string());
                            }
                            if let Some(conclusion) = update.value_of("conclusion") {
                                new.conclusion(conclusion.to_string());
                            }
                            if let Some(on) = update.value_of("on") {
                                match DateTime::parse_from_rfc3339(on) {
                                    Ok(d) => {
                                        new.on(d.timestamp());
                                    }
                                    Err(e) => {
                                        eprintln!("can't parse date-time -> {}", e);
                                    }
                                }
                            }
                            if let Some(priority) = update.value_of("priority") {
                                match priority.parse::<u8>() {
                                    Ok(p) => {
                                        new.priority(p);
                                    }
                                    Err(e) => {
                                        eprintln!("{} priority must be in range [0-255]", e);
                                    }
                                }
                            }
                            if are_you_sure(&format!(
                                "about replace this: \n{:#}\nwith: \n{:#}\n",
                                old, new
                            )) {
                                let new: Box<dyn Storable> = Box::new(new);
                                let old: Box<dyn Storable> = Box::new(old);
                                store.replace::<Note>(&old, new);
                                save(&mut store);
                            }
                        } else {
                            eprintln!("no result found.");
                        }
                    }
                    None => {
                        eprintln!("no note have been saved.");
                    }
                }
            }
            Some(("list", list)) => {
                if let Some(pattern) = list.value_of("pattern") {
                    match store.find::<Note>(pattern) {
                        Some(res) => res.iter().for_each(|x| {
                            println!("{:#}", serde_json::from_str::<Note>(x).unwrap())
                        }),
                        None => eprintln!("There is no note with this pattern."),
                    }
                } else {
                    match store.find::<Note>("\"") {
                        Some(res) => res.iter().for_each(|x| {
                            println!("{:#}", serde_json::from_str::<Note>(x).unwrap())
                        }),
                        None => eprintln!("There is no note."),
                    }
                }
            }
            Some(("remove", remove)) => {
                let force = remove.is_present("force-remove");
                match remove.occurrences_of("pattern") {
                    0 => {
                        if !force {
                            if are_you_sure("delete all notes") {
                                match store.remove_all::<Note>() {
                                    Some(deleted) => {
                                        println!("followings are deleted: ");
                                        deleted.iter().for_each(|d| println!("{:#}", d));
                                        save(&mut store);
                                    }
                                    None => eprintln!("no notes to delete."),
                                };
                            }
                        } else {
                            store.remove_all::<Note>();
                            save(&mut store);
                        }
                    }
                    _ => {
                        let patterns = remove.values_of("pattern").unwrap();
                        let mut to_delete: Vec<Box<dyn Storable>> = Vec::new();
                        patterns.for_each(|pat| match store.find::<Note>(pat) {
                            Some(res) => res.iter().for_each(|x| {
                                if !force {
                                    println!("-> {}", x);
                                }
                                to_delete.push(Box::new(serde_json::from_str::<Note>(x).unwrap()));
                            }),
                            None => {
                                if !force {
                                    eprintln!("no results for '{}'", pat);
                                }
                            }
                        });
                        if !to_delete.is_empty() {
                            if !force {
                                if are_you_sure("delete all of above") {
                                    store.remove_vec::<Note>(to_delete);
                                    save(&mut store);
                                }
                            } else {
                                store.remove_vec::<Note>(to_delete);
                                save(&mut store);
                            }
                        }
                    }
                }
            }
            _ => unreachable!(),
        },
        Some(("todo", todo)) => match todo.subcommand() {
            Some(("import", import)) => {
                let file = import.value_of("JSONFILE").unwrap();
                match std::fs::canonicalize(file) {
                    Ok(path) => {
                        println!("parsing {:?}", path);
                        match serde_json::from_str::<Vec<Todo>>(&Store::load_file(path).unwrap()) {
                            Ok(vec) => store.add_vec::<Todo>(vec),
                            Err(e) => eprintln!("{} is not a json array of todo! -> {:?}", file, e),
                        };
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
            }
            Some(("create", create)) => {
                let mut t = Todo::new();
                t.title(create.value_of("title").unwrap().to_string());
                if let Some(due) = create.value_of("due") {
                    match DateTime::parse_from_rfc3339(due) {
                        Ok(d) => {
                            t.due(d.timestamp());
                        }
                        Err(e) => {
                            eprintln!("can't parse date-time -> {}", e);
                        }
                    }
                }
                if let Some(tag) = create.value_of("tag") {
                    t.tag(tag.to_string());
                }
                if let Some(cause) = create.value_of("cause") {
                    t.cause(cause.to_string());
                }
                if let Some(done) = create.value_of("done") {
                    t.done(done.parse().unwrap());
                }
                if let Some(priority) = create.value_of("priority") {
                    match priority.parse::<u8>() {
                        Ok(p) => {
                            t.priority(p);
                        }
                        Err(e) => {
                            eprintln!("{} priority must be in range [0-255]", e);
                        }
                    }
                }
                store.add::<Todo>(t);
                save(&mut store);
            }
            Some(("update", update)) => {
                match store.find::<Todo>(update.value_of("pattern").unwrap()) {
                    Some(res) => {
                        if res.len() > 0 {
                            let old = serde_json::from_str::<Todo>(&res.first().unwrap()).unwrap();
                            let mut new =
                                serde_json::from_str::<Todo>(&res.first().unwrap()).unwrap();
                            if let Some(title) = update.value_of("title") {
                                new.title(title.to_string());
                            }
                            if let Some(tag) = update.value_of("tag") {
                                new.tag(tag.to_string());
                            }
                            if let Some(cause) = update.value_of("cause") {
                                new.cause(cause.to_string());
                            }
                            if let Some(due) = update.value_of("due") {
                                match DateTime::parse_from_rfc3339(due) {
                                    Ok(d) => {
                                        new.due(d.timestamp());
                                    }
                                    Err(e) => {
                                        eprintln!("can't parse date-time -> {}", e);
                                    }
                                }
                            }
                            if let Some(priority) = update.value_of("priority") {
                                match priority.parse::<u8>() {
                                    Ok(p) => {
                                        new.priority(p);
                                    }
                                    Err(e) => {
                                        eprintln!("{} priority must be in range [0-255]", e);
                                    }
                                }
                            }
                            if let Some(done) = update.value_of("done") {
                                new.done(done.parse().unwrap());
                            }
                            if are_you_sure(&format!(
                                "about replace this: \n{:#}\nwith: \n{:#}\n",
                                old, new
                            )) {
                                let new: Box<dyn Storable> = Box::new(new);
                                let old: Box<dyn Storable> = Box::new(old);
                                store.replace::<Todo>(&old, new);
                                save(&mut store);
                            }
                        } else {
                            eprintln!("no result found.");
                        }
                    }
                    None => {
                        eprintln!("no note have been saved.");
                    }
                }
            }
            Some(("list", list)) => {
                if let Some(pattern) = list.value_of("pattern") {
                    match store.find::<Todo>(pattern) {
                        Some(res) => res.iter().for_each(|x| {
                            println!("{:#}", serde_json::from_str::<Todo>(x).unwrap())
                        }),
                        None => eprintln!("There is no todo with this pattern."),
                    }
                } else {
                    match store.find::<Todo>("\"") {
                        Some(res) => res.iter().for_each(|x| {
                            println!("{:#}", serde_json::from_str::<Todo>(x).unwrap())
                        }),
                        None => eprintln!("There is no todo."),
                    }
                }
            }
            Some(("remove", remove)) => {
                let force = remove.is_present("force-remove");
                match remove.occurrences_of("pattern") {
                    0 => {
                        if !force {
                            if are_you_sure("delete all todo list") {
                                match store.remove_all::<Todo>() {
                                    Some(deleted) => {
                                        println!("followings are deleted: ");
                                        deleted.iter().for_each(|d| println!("{:#}", d));
                                        save(&mut store);
                                    }
                                    None => eprintln!("no todo to delete."),
                                };
                            }
                        } else {
                            store.remove_all::<Todo>();
                            save(&mut store);
                        }
                    }
                    _ => {
                        let patterns = remove.values_of("pattern").unwrap();
                        let mut to_delete: Vec<Box<dyn Storable>> = Vec::new();
                        patterns.for_each(|pat| match store.find::<Todo>(pat) {
                            Some(res) => res.iter().for_each(|x| {
                                if !force {
                                    println!("-> {}", x);
                                }
                                to_delete.push(Box::new(serde_json::from_str::<Todo>(x).unwrap()));
                            }),
                            None => {
                                if !force {
                                    eprintln!("no results for '{}'", pat);
                                }
                            }
                        });
                        if !to_delete.is_empty() {
                            if !force {
                                if are_you_sure("delete all of above") {
                                    store.remove_vec::<Todo>(to_delete);
                                    save(&mut store);
                                }
                            } else {
                                store.remove_vec::<Todo>(to_delete);
                                save(&mut store);
                            }
                        }
                    }
                }
            }
            Some(("check", check)) => match check.occurrences_of("pattern") {
                0 => {
                    if are_you_sure("check all todo list as done") {
                        match store.find::<Todo>("\"") {
                            Some(res) => res.iter().for_each(|x| {
                                let old = serde_json::from_str::<Todo>(x).unwrap();
                                let mut new = serde_json::from_str::<Todo>(x).unwrap();
                                new.done(true);
                                let old: Box<dyn Storable> = Box::new(old);
                                let new: Box<dyn Storable> = Box::new(new);
                                store.replace::<Todo>(&old, new);
                            }),
                            None => eprintln!("There is no todo."),
                        };
                        save(&mut store);
                    }
                }
                _ => {
                    check.values_of("pattern").unwrap().for_each(|pat| {
                        match store.find::<Todo>(pat) {
                            Some(res) => res.iter().for_each(|x| {
                                let old = serde_json::from_str::<Todo>(x).unwrap();
                                let mut new = serde_json::from_str::<Todo>(x).unwrap();
                                new.done(true);
                                let old: Box<dyn Storable> = Box::new(old);
                                let new: Box<dyn Storable> = Box::new(new);
                                store.replace::<Todo>(&old, new);
                            }),
                            None => eprintln!("no results for '{}'", pat),
                        }
                    });
                    save(&mut store);
                }
            },
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };
}
