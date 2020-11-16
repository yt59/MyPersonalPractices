use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct Note {
    subject: String,
    time: i64,
    explanation: String,
    conclusion: String,
    created: i64,
    priority: u8,
    repeat: u8,
}
impl Note {
    pub fn new() -> Note {
        let mut note = Note {
            subject: String::new(),
            time: chrono::Local::now().timestamp(),
            explanation: String::new(),
            conclusion: String::new(),
            created: chrono::Local::now().timestamp(),
            priority: 0,
            repeat: 1,
        };
        note
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
    pub fn time(&mut self, time: i64) -> &mut Self {
        if time > 0 {
            self.time = time;
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
}
#[derive(Serialize, Deserialize)]
struct Todo {
    title: String,
    tag: String,
    created: i64,
    cause: String,
    due: i64,
    priority: u8,
    done: bool,
}
impl Todo {
    pub fn new() -> Todo {
        let mut todo = Todo {
            title: String::new(),
            tag: String::new(),
            cause: String::new(),
            created: chrono::Local::now().timestamp(),
            due: chrono::Local::now().timestamp() + 3600,
            priority: 0,
            done: false,
        };
        todo
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
}

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
    #[test]
    fn test_read_from_file() {
        let name = "test.txt";
        match super::read_from_file(name) {
            Ok(_s) => (),
            Err(_) => panic!(),
        }
    }
    #[test]
    fn test_write_read_to_file() {
        let data = String::from("test data!");
        let name = "test.txt";
        match super::write_to_file(data.clone(), name) {
            Err(_e) => panic!(),
            _ => (),
        }
        match super::read_from_file(name) {
            Ok(s) if s == data => {
                std::fs::remove_file(dirs::home_dir().unwrap().join(".va").join(name)).unwrap();
            }
            _ => panic!(),
        }
    }
}
