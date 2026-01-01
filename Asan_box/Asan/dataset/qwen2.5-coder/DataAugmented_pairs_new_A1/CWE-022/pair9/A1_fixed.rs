use std::ffi::CString;
use std::path::{Path, PathBuf};
use std::thread;

struct Manager {
    base: PathBuf,
}

impl Manager {
    fn new(base: &Path) -> Self {
        Manager {
            base: base.to_path_buf(),
        }
    }

    fn get_path(&self, input: &str) -> Result<PathBuf, String> {
        let joined = self.base.join(input);

        // Normalize the path to prevent path traversal
        let normalized = joined.canonicalize().map_err(|_| "Error: Path canonicalization failed")?;

        if !normalized.starts_with(&self.base) {
            return Err("Error: Path traversal detected".to_string());
        }

        Ok(normalized)
    }
}

fn main() {
    let safe = std::path::Path::new("safe_dir");
    let instance = Manager::new(safe);
    let input = "../secret.txt";
    let handle = thread::spawn({
        let inst = instance;
        move || {
            match inst.get_path(input) {
                Ok(path) => println!("Computed path: {:?}", path),
                Err(e) => println!("Error: {}", e),
            }
        }
    });
    handle.join().unwrap();
}