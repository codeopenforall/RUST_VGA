use std::fs;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

pub struct FileHandler {
    base: PathBuf,
}

impl FileHandler {
    pub fn new(base: &str) -> Self {
        FileHandler {
            base: PathBuf::from(base),
        }
    }

    pub fn get_content(&self, input: &str) -> Result<String, Error> {
        let path = self.base.join(input);
        let s = path.as_os_str().to_str().unwrap();
        fs::read_to_string(path)
    }
}