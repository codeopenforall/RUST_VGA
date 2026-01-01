use std::fs;
use std::path::{Path, PathBuf};

pub struct FileManager {
    base: PathBuf,
}

impl FileManager {
    pub fn new(base: PathBuf) -> Self {
        FileManager { base }
    }

    pub fn open_file(&self, user_input: &str) -> Result<String, String> {
        let joined_path = self.base.join(user_input);
        let canonical_base = fs::canonicalize(&self.base)
            .map_err(|e| format!("Error canonicalizing base: {:?}", e))?;
        let canonical_path = fs::canonicalize(&joined_path)
            .map_err(|e| format!("Error canonicalizing path: {:?}", e))?;
        if !canonical_path.starts_with(&canonical_base) {
            return Err("Access to the requested file is not allowed".to_string());
        }
        let path_str = canonical_path.to_str().ok_or_else(|| "Invalid path".to_string())?;
        let contents = fs::read_to_string(path_str)
            .map_err(|e| format!("Error reading file: {:?}", e))?;
        Ok(contents)
    }
}