use chrono::{DateTime, Local, NaiveDateTime, Utc};
use serde::{de, Deserialize, Serialize, Serializer};
use serde_traitobject as s;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;
use std::any::Any;

#[derive(Serialize, Deserialize, Debug)]
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
    fn debug_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }

    fn display_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }

    // fn get_serialzer(&self){
    //     Serialize::serialize(self, S)
    // }
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
    fn debug_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }

    fn display_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

pub trait Storable: s::Serialize + s::Deserialize + Any + 'static {
    fn debug_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    fn display_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    // fn get_serialzer<S: Serializer>(&self, serializer: &S) -> Result<S::Ok, S::Error>;
}
impl Debug for dyn Storable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.debug_fmt(f)
    }
}
impl Display for dyn Storable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display_fmt(f)
    }
}
// impl Serialize for dyn Storable{
//     fn serialize(&self, serializer: Serializer::) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer {
//             self.get_serialzer(serializer)
//     }
// }

#[derive(Serialize, Deserialize)]
pub(crate) struct Store{
    // #[serde(with = "serde_traitobject")]
    data: HashMap<String, Vec<s::Box<dyn Storable>>>,
}
impl Store {
    // pub fn new() -> Self {
    //     Self(HashMap::new())
    // }
    // pub fn add<S: Storable+'static>(&mut self, data: S) {
    //     let key = std::any::type_name::<S>().to_string();
    //     if self.0.contains_key(&key) {
    //         self.0.get_mut(&key).unwrap().push(Box::new(data))
    //     } else {
    //         self.0.insert(key, vec![Box::new(data)]);
    //     }
    // }
    pub fn show(&self) {
        // println!("{:#?}", self);
    }
}

// impl Store {
//     pub fn load() -> Result<Store, std::io::Error> {
//         match read_from_file("store.json") {
//             Ok(data) => Ok(serde_json::from_str(&data).unwrap_or_else(|_| -> Store {
//                 eprintln!("ERROR: FAILED TO PARSE 'store.json'");
//                 std::fs::rename(
//                     dirs::home_dir().unwrap().join(Path::new(".va/store.json")),
//                     dirs::home_dir()
//                         .unwrap()
//                         .join(Path::new(".va/store.json_ERR")),
//                 )
//                 .unwrap();
//                 println!("LOG: old file renamed to store.json_ERR");
//                 Store {
//                     todo: Vec::new(),
//                     note: Vec::new(),
//                 }
//             })),
//             Err(e) => Err(e),
//         }
//     }
//     pub fn save(&self) -> Result<(), std::io::Error> {
//         let data = serde_json::to_string(&self).unwrap();
//         write_to_file(data, "store.json")
//     }
//     pub fn add<T>(&mut self, note: T) {
//         self.note.push(note);
//     }
//     pub fn add_vec_note(&mut self, notes: Vec<Note>) {
//         todo!()
//     }
//     pub fn find_note(&self, pattern: &str) -> &Note {
//         todo!()
//     }
//     pub fn find_all_note(&self, pattern: &str) -> Vec<&Note> {
//         todo!()
//     }
//     pub fn remove_note(&mut self, note: &Note) -> Option<Note> {
//         todo!()
//     }
//     pub fn remove_vec_note(&mut self, notes: Vec<&Note>) -> Vec<Note> {
//         todo!()
//     }
// }

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
    // #[test]
    // fn test_load_store() {
    //     Store::load().unwrap();
    // }
    // #[test]
    // fn test_load_save_store() {
    //     Store::load().unwrap().save().unwrap()
    // }
}
