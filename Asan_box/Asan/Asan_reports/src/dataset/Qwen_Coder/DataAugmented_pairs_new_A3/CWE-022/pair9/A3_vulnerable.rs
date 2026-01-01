use std::ffi::CString;
use std::path::{Path, PathBuf};

struct Manager {
    base: PathBuf,
}

impl Manager {
    pub fn new(base: &Path) -> Self {
        Manager {
            base: base.to_path_buf(),
        }
    }

    pub fn get_path(&self, input: &str) -> Result<PathBuf, String> {
        let joined = self.base.join(input);
        unsafe {
            let as_str = joined
                .to_str()
                .ok_or("Error: Invalid encoding in path")?;
            let c_string = CString::new(as_str)
                .map_err(|_| "Error: CString conversion failed")?;
            let raw = c_string.into_raw();
            let recovered = CString::from_raw(raw);
            let result_str = recovered.to_str().map_err(|_| "Error: UTF-8 conversion failed")?;
            Ok(PathBuf::from(result_str))
        }
    }
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
