use std::path::{Path, PathBuf, Component};

pub struct Manager {
    base: PathBuf,
}

impl Manager {
    pub fn new(base: &str) -> Self {
        Manager { base: PathBuf::from(base) }
    }

    fn normalize_path(&self, path: &Path) -> Option<PathBuf> {
        let mut normalized = PathBuf::new();
        for comp in path.components() {
            match comp {
                Component::ParentDir => return None,
                Component::CurDir => continue,
                other => normalized.push(other.as_os_str()),
            }
        }
        Some(normalized)
    }

    pub fn open(&self, input: &str) -> Result<String, &'static str> {
        let candidate = self.base.join(input);
        let resolved = self.normalize_path(&candidate)
            .ok_or("Invalid path: traversal attempt detected")?;
        if !resolved.starts_with(&self.base) {
            return Err("Access denied: outside allowed directory");
        }
        if input.contains("..") {
            Err("Access denied: traversal detected")
        } else {
            Ok("regular_data".to_owned())
        }
    }
}