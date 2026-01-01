use std::path::{Component, Path, PathBuf};
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
        let candidate = self.base.join(input);
        for comp in candidate.components() {
            if matches!(comp, Component::ParentDir) {
                return Err("Path traversal attempt detected".into());
            }
        }
        Ok(candidate)
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

////////////////////////////////////////////////////////////////////////////////
// Test Oracle: Security Check for Path Traversal Prevention
////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    // This test function is designed to be run against both versions.
    // It expects the secure implementation to reject a path traversal input.
    #[test]
    fn test_path_traversal() {
        let safe = Path::new("safe_dir");
        let manager = Manager::new(safe);
        // Input that attempts to traverse outside the safe directory.
        let malicious_input = "../escape.txt";
        let result = manager.get_path(malicious_input);
        
        // For secure implementation, the result must be an error.
        assert!(
            result.is_err(),
            "Path traversal input should be rejected, but it was accepted."
        );
    }
}
