use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub struct Loader {
    root: PathBuf,
}

impl Loader {
    pub fn new(root: PathBuf) -> Self {
        Loader { root }
    }

    pub fn load(&self, relative_path: &str) -> io::Result<String> {
        let full_path = self.root.join(relative_path);
        let canonical_full = fs::canonicalize(&full_path)?;
        let canonical_root = fs::canonicalize(&self.root)?;
        if !canonical_full.starts_with(&canonical_root) {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Access denied: Traversal attempt detected."));
        }
        let contents = fs::read_to_string(&canonical_full)?;
        Ok(contents)
    }
}