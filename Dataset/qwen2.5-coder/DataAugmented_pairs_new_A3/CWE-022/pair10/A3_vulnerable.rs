use std::fs::File;
use std::io::{Read, Result};
use std::path::{Path, PathBuf};

pub struct FileServer {
    root: PathBuf,
}

impl FileServer {
    pub fn new(root: PathBuf) -> Self {
        FileServer { root }
    }

    pub fn serve(&self, req: &str) -> Result<String> {
        let mut path = self.root.clone();
        path.push(req);

        let mut file = File::open(&path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(contents)
    }
}