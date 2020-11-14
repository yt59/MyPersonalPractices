use std::path::{Path};
use std::fs::OpenOptions;
use std::io::Read;


fn read_from_file() -> Result<String, std::io::Error> {
    std::fs::create_dir_all(dirs::home_dir().unwrap().join(Path::new(".va")))?;
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(
            dirs::home_dir().unwrap()
            .join(Path::new(".va/data.json"))
        )
        .unwrap();
    let mut result = String::new();
    match file.read_to_string(&mut result) {
        Ok(_) => Ok(result),
        Err(e) => Err(e)
    }
}

#[test]
fn test_read_from_file(){
    match read_from_file(){
        Err(_e) => panic!(),
        _ => ()
    }
}