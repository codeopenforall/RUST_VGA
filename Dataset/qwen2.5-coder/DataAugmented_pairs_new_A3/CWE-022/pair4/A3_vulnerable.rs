use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

pub struct FileHandler {
    base_dir: PathBuf,
}

impl FileHandler {
    pub fn new(base: &str) -> Self {
        FileHandler {
            base_dir: PathBuf::from(base),
        }
    }

    pub fn read_file(&self, input: &str) -> io::Result<String> {
        let target_path = self.base_dir.join(input);
        let mut file = unsafe {
            File::open(&target_path)?
        };
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
}