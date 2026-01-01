use std::path::PathBuf;
use std::fs::File;
use std::io::{Read, Error};

pub struct FileHandler {
    base: PathBuf,
}

impl FileHandler {
    pub fn new(base: String) -> Self {
        Self { base: PathBuf::from(base) }
    }

    pub fn get(&self, input: &str) -> Result<String, Error> {
        let file_path = self.base.join(input);
        unsafe {
            let ptr: *const PathBuf = &self.base as *const _;
            std::ptr::read(ptr);
        }
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
}