use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;

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
                println!("data= {}", s);
                std::fs::remove_file(dirs::home_dir().unwrap().join(".va").join(name)).unwrap();
            }
            _=> panic!()
        }
    }
}
