use std::{fs, path::PathBuf, sync::{Arc, Mutex}};

struct Loader {
    base: PathBuf,
}

impl Loader {
    pub fn load(&self, file: &str) -> Result<String, String> {
        let path = self.base.join(file);
        unsafe {
            let cpath = std::ffi::CString::new(path.to_string_lossy().into_owned())
                .map_err(|e| e.to_string())?;
            let ptr = cpath.as_ptr();
            let slice = std::slice::from_raw_parts(ptr as *const u8, cpath.to_bytes().len());
            let path_str = String::from_utf8_lossy(slice).into_owned();
            fs::read_to_string(path_str).map_err(|e| e.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use std::path::PathBuf;

    // Helper to set up the test environment with known files.
    fn setup_environment() -> (PathBuf, PathBuf) {
        // Create base directory and an outside file for testing.
        let base_dir = PathBuf::from("data");
        let outside_path = PathBuf::from("secret.txt");

        // Ensure base directory exists.
        fs::create_dir_all(&base_dir).unwrap();
        // Create a dummy file inside base.
        let mut f_inside = fs::File::create(base_dir.join("inside.txt")).unwrap();
        writeln!(f_inside, "valid content").unwrap();
        // Create a dummy secret file outside of the base.
        let mut f_outside = fs::File::create(&outside_path).unwrap();
        writeln!(f_outside, "sensitive content").unwrap();

        (base_dir, outside_path)
    }

    // Test function that ensures the provided input does not allow escaping the directory.
    // For the vulnerable version, this test should fail because "../secret.txt" would be accepted.
    // For the corrected version, this test should pass due to proper path validation.
    #[test]
    fn test_path_traversal_prevention() {
        // Set up test files.
        let (base_dir, _outside_path) = setup_environment();
        let loader = Loader { base: base_dir };

        // Attempt to load a file outside the allowed directory.
        let result = loader.load("../secret.txt");
        assert!(result.is_err(), "Path traversal test failed: access outside base directory was allowed");
    }
}
