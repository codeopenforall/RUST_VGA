use std::path::PathBuf;
use std::fs::File;
use std::io::{Read, Error};

pub struct FileHandler {
    base: PathBuf,
}

impl FileHandler {
    pub fn new(base: String) -> Self {
        Self { base: PathBuf::from(base) }
    }

    pub fn get(&self, input: &str) -> Result<String, Error> {
        let file_path = self.base.join(input);
        unsafe {
            let ptr: *const PathBuf = &self.base as *const _;
            std::ptr::read(ptr);
        }
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
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
