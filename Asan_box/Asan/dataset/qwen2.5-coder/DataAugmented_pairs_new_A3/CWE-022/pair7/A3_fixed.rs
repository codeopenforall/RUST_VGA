use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub struct Loader {
    base: PathBuf,
}

impl Loader {
    pub fn new(base: &str) -> Self {
        let canonical_base = fs::canonicalize(base).expect("Invalid base directory");
        Loader { base: canonical_base }
    }

    pub fn load_file(&self, path: &str) -> io::Result<String> {
        let full_path = self.base.join(path);
        if !self.is_allowed(&full_path) {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "Path traversal detected",
            ));
        }
        fs::read_to_string(&full_path)
    }

    fn is_allowed(&self, candidate: &Path) -> bool {
        if let Ok(canonical_candidate) = candidate.canonicalize() {
            canonical_candidate.starts_with(&self.base)
        } else {
            false
        }
    }
}