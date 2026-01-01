use std::fs::File;
use std::io::{Read, Error as IoError};
use std::path::PathBuf;

pub struct FileHandler {
    root: PathBuf,
}

impl FileHandler {
    pub fn new(root: PathBuf) -> Self {
        FileHandler { root }
    }

    pub fn fetch(&self, user_input: &str) -> Result<String, IoError> {
        let joined = self.root.join(user_input);
        let mut file = File::open(joined)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        println!("Main thread read: {}", &data[0..std::cmp::min(20, data.len())]);
        Ok(data)
    }
}