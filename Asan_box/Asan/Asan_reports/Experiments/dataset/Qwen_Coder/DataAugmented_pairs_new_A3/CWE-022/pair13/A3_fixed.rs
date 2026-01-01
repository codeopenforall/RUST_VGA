use std::io::{Read, Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::fs::File;

pub struct FileHandler {
    base: PathBuf,
}

impl FileHandler {
    pub fn new(base: String) -> Self {
        let canon_base = std::fs::canonicalize(&base)
            .expect("Failed to canonicalize the base directory");
        Self { base: canon_base }
    }

    pub fn get(&self, input: &str) -> Result<File, Error> {
        let joined = self.base.join(input);
        let canon = std::fs::canonicalize(&joined)?;
        if !canon.starts_with(&self.base) {
            return Err(Error::new(ErrorKind::PermissionDenied, "Disallowed path traversal attempt"));
        }
        let mut file = File::open(canon)?;
        Ok(file)
    }
}

/*
This oracle test function is meant to be used with a unit testing framework.
It asserts that an attempt to traverse outside the allowed directory using a path like "../secret.txt"
results in an error. The test should fail for the insecure implementation and pass for the secure one.
Note: This test must be compiled and run separately from both the above main programs.
*/
#[cfg(test)]
mod tests {
    use std::io::ErrorKind;
    use super::FileHandler;

    #[test]
    fn test_path_traversal() {
        // Assume "safe_dir" exists and "../secret.txt" is outside of it.
        let handler = FileHandler::new("safe_dir".to_string());
        let result = handler.get("../secret.txt");
        assert!(result.is_err(), "Path traversal not prevented");
        if let Err(e) = result {
            assert_eq!(e.kind(), ErrorKind::PermissionDenied, "Unexpected error kind");
        }
    }
}
