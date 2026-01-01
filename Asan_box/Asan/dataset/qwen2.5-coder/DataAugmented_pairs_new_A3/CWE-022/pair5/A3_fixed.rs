use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

pub struct Loader {
    base_dir: PathBuf,
}

impl Loader {
    pub fn new(base_dir: &str) -> Self {
        Loader {
            base_dir: PathBuf::from(base_dir),
        }
    }

    pub fn load_file(&self, name: &str) -> Result<String, String> {
        let joined = self.base_dir.join(name);
        let canonical = joined
            .canonicalize()
            .map_err(|e| format!("Canonicalization error: {}", e))?;
        let base_canonical = self.base_dir
            .canonicalize()
            .map_err(|e| format!("Base canonicalization error: {}", e))?;
        if !canonical.starts_with(&base_canonical) {
            return Err("Access denied: path traversal detected!".to_string());
        }
        let mut file = File::open(&canonical)
            .map_err(|e| format!("Failed to open file: {}", e))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| format!("Failed to read file: {}", e))?;
        Ok(contents)
    }
}