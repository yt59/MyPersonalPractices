use chrono::{DateTime, Local, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::{
    any::Any,
    fmt::{Debug, Display},
};
use std::{collections::HashMap, collections::HashSet, hash::Hash};

#[derive(Serialize, Deserialize, Debug, Hash)]
pub(crate) struct Note {
    subject: String,
    on: i64,
    explanation: String,
    conclusion: String,
    created: i64,
    priority: u8,
}
#[allow(dead_code)]
impl Note {
    pub fn new() -> Self {
        Note {
            subject: String::new(),
            on: Local::now().timestamp(),
            explanation: String::new(),
            conclusion: String::new(),
            created: Local::now().timestamp(),
            priority: 0,
        }
    }
    pub fn subject(&mut self, subject: String) -> &mut Self {
        if subject.len() > 0 {
            self.subject = subject;
        }
        self
    }
    pub fn explanation(&mut self, explanation: String) -> &mut Self {
        if explanation.len() > 0 {
            self.explanation = explanation;
        }
        self
    }
    pub fn conclusion(&mut self, conclusion: String) -> &mut Self {
        if conclusion.len() > 0 {
            self.conclusion = conclusion;
        }
        self
    }
    pub fn on(&mut self, on: i64) -> &mut Self {
        if on > 0 {
            self.on = on;
        }
        self
    }
    pub fn priority(&mut self, priority: u8) -> &mut Self {
        self.priority = priority;
        self
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tz = Local::now().timezone();
        f.debug_struct("Note")
            .field("subject", &self.subject)
            .field(
                "on",
                &DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(self.on, 0), Utc)
                    .with_timezone(&tz),
            )
            .field("explanation", &self.explanation)
            .field("conclusion", &self.conclusion)
            .field("priority", &self.priority)
            .field(
                "created on",
                &DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(self.on, 0), Utc)
                    .with_timezone(&tz),
            )
            .finish()
    }
}
impl Storable for Note {
    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Todo {
    title: String,
    tag: String,
    created: i64,
    cause: String,
    due: i64,
    priority: u8,
    done: bool,
}
#[allow(dead_code)]
impl Todo {
    pub fn new() -> Self {
        Todo {
            title: String::new(),
            tag: String::new(),
            cause: String::new(),
            created: Local::now().timestamp(),
            due: Local::now().timestamp() + 3600,
            priority: 0,
            done: false,
        }
    }
    pub fn title(&mut self, title: String) -> &mut Self {
        if title.len() > 0 {
            self.title = title;
        }
        self
    }
    pub fn tag(&mut self, tag: String) -> &mut Self {
        if tag.len() > 0 {
            self.tag = tag;
        }
        self
    }
    pub fn cause(&mut self, cause: String) -> &mut Self {
        if cause.len() > 0 {
            self.cause = cause;
        }
        self
    }
    pub fn due(&mut self, due: i64) -> &mut Self {
        if due > self.created {
            self.due = due;
        }
        self
    }
    pub fn priority(&mut self, priority: u8) -> &mut Self {
        self.priority = priority;
        self
    }
    pub fn done(&mut self, done: bool) -> &mut Self {
        self.done = done;
        self
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tz = Local::now().timezone();
        f.debug_struct("Note")
            .field("title", &self.title)
            .field(
                "due",
                &DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(self.due, 0), Utc)
                    .with_timezone(&tz),
            )
            .field("cause", &self.cause)
            .field("tag", &self.tag)
            .field("done", &self.done)
            .field("priority", &self.priority)
            .field(
                "created on",
                &DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(self.created, 0), Utc)
                    .with_timezone(&tz),
            )
            .finish()
    }
}

impl Storable for Todo {
    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

pub trait Storable: Debug + Display {
    fn to_json(&self) -> String;
}

impl Hash for Box<dyn Storable> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_string().hash(state)
    }
}
impl PartialEq for Box<dyn Storable> {
    fn eq(&self, other: &Self) -> bool {
        self.to_string().eq(&other.to_string())
    }
}
impl Eq for Box<dyn Storable> {
    fn assert_receiver_is_total_eq(&self) {}
}

#[derive(Debug)]
pub(crate) struct Store(HashMap<String, HashSet<Box<dyn Storable>>>);

#[allow(dead_code)]
impl Store {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    fn _load_note(
        hs: &mut HashSet<Box<dyn Storable>>,
        raw: Vec<String>,
    ) -> Result<(), serde_json::Error> {
        raw.into_iter()
            .map(|s| match serde_json::from_str::<Note>(&s) {
                Ok(v) => {
                    hs.insert(Box::new(v));
                    Ok(())
                }
                Err(e) => Err(e),
            })
            .take_while(|x| x.is_ok())
            .fold(Ok(()), |acc, x| if acc.is_err() { acc } else { x })
    }
    fn _load_todo(
        hs: &mut HashSet<Box<dyn Storable>>,
        raw: Vec<String>,
    ) -> Result<(), serde_json::Error> {
        raw.into_iter()
            .map(|s| match serde_json::from_str::<Todo>(&s) {
                Ok(v) => {
                    hs.insert(Box::new(v));
                    Ok(())
                }
                Err(e) => Err(e),
            })
            .take_while(|x| x.is_ok())
            .fold(Ok(()), |acc, x| if acc.is_err() { acc } else { x })
    }
    fn load_value(
        key: &String,
        raw: Vec<String>,
    ) -> Result<HashSet<Box<dyn Storable>>, serde_json::Error> {
        let mut value: HashSet<Box<dyn Storable>> = HashSet::new();
        match match &key[..] {
            "va::data_store::Note" => Store::_load_note(&mut value, raw),
            "va::data_store::Todo" => Store::_load_todo(&mut value, raw),
            _ => panic!("UNKNOWN KEY!")
        } {
            Ok(_) => Ok(value),
            Err(e) => Err(e),
        }
    }
    fn load_store(hm: HashMap<String, Vec<String>>) -> Result<Store, serde_json::Error> {
        let mut store = Store(HashMap::new());
        match hm
            .into_iter()
            .map(|(k, v)| match Store::load_value(&k, v) {
                Ok(value) => {
                    store.0.insert(k, value);
                    Ok(())
                }
                Err(e) => Err(e),
            })
            .take_while(|x| x.is_ok())
            .fold(Ok(()), |acc, x| if acc.is_err() { acc } else { x })
        {
            Ok(_) => Ok(store),
            Err(e) => Err(e),
        }
    }
    fn load_file(path: PathBuf) -> Result<String, std::io::Error> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .unwrap();
        let mut result = String::new();
        match file.read_to_string(&mut result) {
            Ok(_) => Ok(result),
            Err(e) => Err(e),
        }
    }
    pub fn load() -> Result<Store, std::io::Error> {
        std::fs::create_dir_all(dirs::home_dir().unwrap().join(Path::new(".va")))?;
        let path = dirs::home_dir().unwrap().join(Path::new(".va/store.json"));
        let data: String = Store::load_file(path)?;
        let eh = |_| {
            eprintln!("ERROR: FAILED TO PARSE 'store.json'");
            std::fs::rename(
                dirs::home_dir().unwrap().join(Path::new(".va/store.json")),
                dirs::home_dir()
                    .unwrap()
                    .join(Path::new(".va/store.json_ERR")),
            )
            .unwrap();
            println!("LOG: old file renamed to store.json_ERR");
        };
        let data = serde_json::from_str::<HashMap<String, Vec<String>>>(&data).unwrap_or_else(
            |e| -> HashMap<String, Vec<String>> {
                eh(e);
                HashMap::new()
            },
        );
        Ok(Store::load_store(data).unwrap_or_else(|e| -> Store {
            eh(e);
            Store(HashMap::new())
        }))
    }
    pub fn save(&self) -> Result<(), std::io::Error> {
        let serializable: HashMap<String, Vec<String>> = self
            .0
            .iter()
            .map(|(key, val)| {
                let ds: Vec<String> = val.iter().map(|data| data.as_ref().to_json()).collect();
                (key.clone(), ds)
            })
            .collect();
        let data = serde_json::to_string(&serializable).unwrap();
        write_to_file(data, "store.json")
    }
    pub fn show(&self) {
        println!("{:#?}", self);
    }
    pub fn add<S: Storable + 'static>(&mut self, data: S) {
        let key = std::any::type_name::<S>().to_string();
        if self.0.contains_key(&key) {
            self.0.get_mut(&key).unwrap().insert(Box::new(data));
        } else {
            let mut value: HashSet<Box<dyn Storable>> = HashSet::new();
            value.insert(Box::new(data));
            self.0.insert(key, value);
        }
    }
    pub fn add_vec<S: Storable + 'static>(&mut self, data: Vec<S>) {
        let key = std::any::type_name::<S>().to_string();
        if self.0.contains_key(&key) {
            data.into_iter().for_each(|n| {
                self.0.get_mut(&key).unwrap().insert(Box::new(n));
            })
        } else {
            let mut value: HashSet<Box<dyn Storable>> = HashSet::new();
            data.into_iter().for_each(|n| {
                value.insert(Box::new(n));
            });
            self.0.insert(key, value);
        }
    }
    pub fn find<S: Storable>(&self, pattern: &str) -> Option<Vec<String>> {
        let key = std::any::type_name::<S>().to_string();
        match self.0.contains_key(&key) {
            true => {
                let mut searchable: HashSet<_> = self
                    .0
                    .get(&key)
                    .unwrap()
                    .iter()
                    .map(|data| data.to_json())
                    .into_iter()
                    .collect();
                searchable.retain(|text| text.contains(pattern));
                let found: Vec<_> = searchable.into_iter().collect();
                match found.is_empty() {
                    true => None,
                    false => Some(found),
                }
            }
            _ => None,
        }
    }
    pub fn remove<S: Storable>(&mut self, data: &Box<dyn Storable>) -> bool {
        let key = std::any::type_name::<S>().to_string();
        if self.0.contains_key(&key) {
            let mut value: HashSet<Box<dyn Storable>> =
                self.0.get_mut(&key).unwrap().drain().collect();
            match value.remove(data) {
                true => {
                    self.0.insert(key, value);
                    true
                }
                false => false,
            }
        } else {
            false
        }
    }
    pub fn remove_vec<S: Storable>(&mut self, data: Vec<&Box<dyn Storable>>) -> bool {
        let key = std::any::type_name::<S>().to_string();
        if self.0.contains_key(&key) {
            let mut value: HashSet<Box<dyn Storable>> =
                self.0.get_mut(&key).unwrap().drain().collect();
            match data.into_iter().map(|d| value.remove(d)).any(|x| x) {
                true => {
                    self.0.insert(key, value);
                    true
                }
                false => {
                    self.0.insert(key, value);
                    false
                }
            }
        } else {
            false
        }
    }
    pub fn remove_all<S: Storable>(&mut self) -> Option<Vec<Box<dyn Storable>>> {
        let key = std::any::type_name::<S>().to_string();
        match self.0.remove(&key) {
            None => None,
            Some(value) => Some(value.into_iter().collect()),
        }
    }
    pub fn replace<S: Storable>(
        &mut self,
        old: &Box<dyn Storable>,
        new: Box<dyn Storable>,
    ) -> bool {
        let key = std::any::type_name::<S>().to_string();
        match self.0.contains_key(&key) {
            false => false,
            true => match self.0.get_mut(&key).unwrap().take(old) {
                Some(_) => {
                    self.0.get_mut(&key).unwrap().insert(new);
                    true
                }
                None => false,
            },
        }
    }
}

#[allow(dead_code)]
fn read_from_file(name: &str) -> Result<String, std::io::Error> {
    std::fs::create_dir_all(dirs::home_dir().unwrap().join(Path::new(".va")))?;
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(
            dirs::home_dir()
                .unwrap()
                .join(Path::new(&format!(".va/{}", name))),
        )
        .unwrap();
    let mut result = String::new();
    match file.read_to_string(&mut result) {
        Ok(_) => Ok(result),
        Err(e) => Err(e),
    }
}
#[allow(dead_code)]
fn write_to_file(data: String, name: &str) -> Result<(), std::io::Error> {
    std::fs::create_dir_all(dirs::home_dir().unwrap().join(Path::new(".va")))?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(
            dirs::home_dir()
                .unwrap()
                .join(Path::new(&format!(".va/{}", name))),
        )
        .unwrap();
    file.write_all(data.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_from_file() {
        let name = "test_read_from_file.txt";
        match read_from_file(name) {
            Ok(_s) => std::fs::remove_file(
                dirs::home_dir()
                    .unwrap()
                    .join(Path::new(&format!(".va/test_read_from_file.txt"))),
            )
            .unwrap(),
            Err(_) => panic!(),
        }
    }
    #[test]
    fn test_write_read_to_file() {
        let data = String::from("test data!");
        let name = "test_write_read_to_file.txt";
        match write_to_file(data.clone(), name) {
            Err(_e) => panic!(),
            _ => (),
        }
        match read_from_file(name) {
            Ok(s) if s == data => {
                std::fs::remove_file(dirs::home_dir().unwrap().join(".va").join(name)).unwrap();
            }
            _ => panic!(),
        }
    }
    #[test]
    fn test_create_note() {
        let mut test = Note::new();
        test.subject("test note".to_string())
            .explanation("it's test note that happened this morning!".to_string())
            .conclusion("TDD is good!".to_string())
            .priority(255);
        let mut array: Vec<&Note> = Vec::new();
        array.push(&test);
        let b_test = serde_json::to_vec(&array).unwrap();
        let res: Vec<Note> = serde_json::from_slice(b_test.as_slice()).unwrap();
        assert_eq!(res.first().unwrap().to_string(), test.to_string());
        assert_eq!(res.first().unwrap().to_json(), test.to_json());
    }
    #[test]
    fn test_create_todo() {
        let mut test = Todo::new();
        test.title("test note".to_string())
            .tag("#urgent".to_string())
            .cause("TDD is good!".to_string())
            .priority(255)
            .due(chrono::Local::now().timestamp())
            .done(true);
        let mut array: Vec<&Todo> = Vec::new();
        array.push(&test);
        let b_test = serde_json::to_vec(&array).unwrap();
        let res: Vec<Todo> = serde_json::from_slice(b_test.as_slice()).unwrap();
        assert_eq!(res.first().unwrap().to_json(), test.to_json());
        assert_eq!(res.first().unwrap().to_string(), test.to_string());
    }
    #[test]
    fn test_load_store() {
        Store::load().unwrap().show();
    }
    #[test]
    fn test_load_save_store() {
        Store::load().unwrap().save().unwrap()
    }
}
