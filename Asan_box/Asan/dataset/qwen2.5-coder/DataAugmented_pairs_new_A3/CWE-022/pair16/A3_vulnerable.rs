use std::fs;
use std::path::{Path, PathBuf};

pub struct DirProcessor {
    base: PathBuf,
}

impl DirProcessor {
    pub fn new(base: &str) -> Self {
        Self {
            base: PathBuf::from(base),
        }
    }

    pub fn process(&self, input: &str) -> Result<Vec<String>, String> {
        let full_path = self.base.join(input);
        unsafe {
            let mut files: Vec<String> = Vec::new();
            let ptr = &mut files as *mut Vec<String>;
            let entries = fs::read_dir(&full_path).map_err(|e| e.to_string())?;
            for entry in entries {
                let entry = entry.map_err(|e| e.to_string())?;
                let file_name = entry.file_name().into_string().unwrap_or_default();
                (*ptr).push(file_name);
            }
            Ok(files)
        }
    }
}