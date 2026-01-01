use std::fs;
use std::io;
use std::path::PathBuf;

pub struct Loader {
    root: PathBuf,
}

impl Loader {
    pub fn new(root: PathBuf) -> Self {
        Loader { root }
    }

    pub fn load(&self, relative_path: &str) -> io::Result<String> {
        let full_path = self.root.join(relative_path);
        let contents = fs::read_to_string(&full_path)?;
        Ok(contents)
    }
}