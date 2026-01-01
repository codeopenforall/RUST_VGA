use std::fs;
use std::path::{Path, PathBuf};

pub struct Manager {
    base: PathBuf,
}

impl Manager {
    pub fn new(base: &str) -> Self {
        Manager {
            base: PathBuf::from(base),
        }
    }

    pub fn process(&self, relative_path: &str) -> Result<String, String> {
        let full_path = self.base.join(relative_path);
        let resolved_path = full_path.canonicalize().map_err(|_| "Invalid path".to_string())?;

        if !resolved_path.starts_with(&self.base) {
            return Err("Access denied: resolved path is outside of the allowed directory".to_string());
        }

        fs::read_to_string(&resolved_path).map_err(|_| "Unable to read file".to_string())
    }
}