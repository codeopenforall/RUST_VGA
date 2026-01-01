use std::fs::{File, canonicalize};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

pub struct FileHandler {
    base_dir: PathBuf,
}

impl FileHandler {
    pub fn new(base: &str) -> Self {
        let base_dir = canonicalize(base).unwrap_or_else(|_| PathBuf::from(base));
        FileHandler { base_dir }
    }

    pub fn read_file(&self, input: &str) -> io::Result<String> {
        let candidate = self.base_dir.join(input);
        let candidate_canon = canonicalize(&candidate)?;
        if !candidate_canon.starts_with(&self.base_dir) {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "Access outside the permitted directory is denied",
            ));
        }
        let mut file = File::open(&candidate_canon)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
}