use std::fs;
use std::path::PathBuf;

pub struct FileManager {
    pub base_dir: PathBuf,
}

impl FileManager {
    pub fn read_content(&self, filename: &str) -> Result<String, String> {
        let target_path = self.base_dir.join(filename);
        fs::read_to_string(&target_path).map_err(|e| format!("Error: {}", e))
    }
}