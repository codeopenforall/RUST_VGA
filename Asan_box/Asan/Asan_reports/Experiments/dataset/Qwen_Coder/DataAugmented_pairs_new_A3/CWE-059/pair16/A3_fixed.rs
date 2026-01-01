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

//////////////////////////////
// File: tests.rs (Test Oracle)
//////////////////////////////
#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::symlink;
    use std::path::Path;
    // Assume that the Manager struct and its implementation are available in the module
    use crate::Manager;

    #[test]
    fn path_validation_oracle() {
        // Setup: Create the safe directory if it does not exist.
        let safe_dir = "safe_dir";
        let _ = fs::create_dir_all(safe_dir);
        // Create a target file outside the safe directory.
        let target_file = "unsafe_target.txt";
        fs::write(target_file, "sensitive information").expect("Unable to write target file");

        // Remove any pre-existing symbolic link.
        let symlink_path = format!("{}/link.txt", safe_dir);
        let _ = fs::remove_file(&symlink_path);
        // Create a symbolic link inside safe_dir that points outside (up one directory).
        symlink(Path::new("../unsafe_target.txt"), &symlink_path)
            .expect("Failed to create symbolic link");

        let mgr = Manager::new(safe_dir);
        let res = mgr.process("link.txt");

        // The oracle expects the file processing to fail if the resolved path is outside safe_dir.
        // For the vulnerable version, res would be Ok (processing the unintended file),
        // causing the test to fail; for the fixed version, res is Err, so the test passes.
        assert!(
            res.is_err(),
            "Test failed: Symbolic link resolution allowed access to an unintended file."
        );
    }
}
