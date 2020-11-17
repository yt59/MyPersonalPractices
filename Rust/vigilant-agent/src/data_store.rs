use chrono::{DateTime, Local, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, std::fmt::Debug)]
struct Note {
    subject: String,
    on: i64,
    explanation: String,
    conclusion: String,
    created: i64,
    priority: u8,
    repeat: u8,
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
            repeat: 1,
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
    pub fn repeat(&mut self, repeat: u8) -> &mut Self {
        if repeat > 1 {
            self.repeat = repeat;
        }
        self
    }
    pub fn add_repeat(&mut self) -> &mut Self {
        self.repeat = self.repeat + 1;
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

impl std::fmt::Display for Note {
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
            .field("repeat", &self.repeat)
            .field("priority", &self.priority)
            .field(
                "created on",
                &DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(self.on, 0), Utc)
                    .with_timezone(&tz),
            )
            .finish()
    }
}

#[derive(Serialize, Deserialize, std::fmt::Debug)]
struct Todo {
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

impl std::fmt::Display for Todo {
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

mod tests {
    use super::*;
    #[test]
    fn test_read_from_file() {
        let name = "test.txt";
        match read_from_file(name) {
            Ok(_s) => (),
            Err(_) => panic!(),
        }
    }
    #[test]
    fn test_write_read_to_file() {
        let data = String::from("test data!");
        let name = "test.txt";
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
}
