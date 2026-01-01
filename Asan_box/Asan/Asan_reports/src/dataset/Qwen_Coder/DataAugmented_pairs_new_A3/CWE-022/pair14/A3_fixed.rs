use std::fs;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

struct FileHandler {
    base: PathBuf,
}

impl FileHandler {
    pub fn new<P: AsRef<Path>>(base: P) -> Self {
        FileHandler {
            base: base.as_ref().to_path_buf(),
        }
    }

    pub fn get_content<P: AsRef<Path>>(&self, input: P) -> Result<String, Error> {
        let joined = self.base.join(input);
        let canon_joined = joined.canonicalize()?;
        let canon_base = self.base.canonicalize()?;
        if !canon_joined.starts_with(&canon_base) {
            return Err(Error::new(ErrorKind::PermissionDenied, "Path traversal attempt detected"));
        }
        let s = canon_joined.as_os_str().to_str().unwrap();
        fs::read_to_string(canon_joined)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::ErrorKind;

    // This test creates a temporary directory as the sandbox.
    // It writes a file outside the allowed directory and then tries to access it
    // via a path traversal attempt. In the corrected version, the attempt is rejected.
    #[test]
    fn path_traversal_prevention() {
        // Setup temporary test directories and files.
        let base_dir = "./test_temp_data";
        let outside_file = "./test_temp_outside.txt";
        fs::create_dir_all(base_dir).unwrap();
        fs::write(format!("{}/allowed.txt", base_dir), "allowed file").unwrap();
        fs::write(outside_file, "secret data").unwrap();

        let handler = FileHandler::new(base_dir);
        // Attempt to traverse outside the base directory
        let result = handler.get_content("../test_temp_outside.txt");

        // For the safe version, the traversal attempt should be blocked
        // yielding a PermissionDenied error.
        assert!(result.is_err(), "Path traversal was not prevented");
        if let Err(err) = result {
            assert_eq!(err.kind(), ErrorKind::PermissionDenied, "Unexpected error kind");
        }

        // Cleanup the temporary test artifacts.
        fs::remove_dir_all(base_dir).unwrap();
        fs::remove_file(outside_file).unwrap();
    }
}
