use std::path::{Component, Path, PathBuf};

struct Manager {
    base: PathBuf,
}

impl Manager {
    fn new(base: &Path) -> Self {
        Manager {
            base: base.to_path_buf(),
        }
    }

    pub fn get_path(&self, input: &str) -> Result<PathBuf, String> {
        let candidate = self.base.join(input);
        for comp in candidate.components() {
            if let Component::ParentDir = comp {
                return Err("Path traversal attempt detected".into());
            }
        }
        Ok(candidate)
    }
}