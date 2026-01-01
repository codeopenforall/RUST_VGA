#![allow(unused)]
use std::fs::File;
use std::io::{Read, Error, ErrorKind};
use std::path::{PathBuf, Component};
use std::sync::{Arc, Mutex};
use std::thread;

struct FileHandler {
    base: PathBuf,
}

impl FileHandler {
    fn new(root: &str) -> Self {
        FileHandler { base: PathBuf::from(root) }
    }

    fn load(&self, input: &str) -> Result<String, Error> {
        let user_path = PathBuf::from(input);
        for comp in user_path.components() {
            if let Component::ParentDir = comp {
                return Err(Error::new(ErrorKind::InvalidInput, "Path traversal is not allowed"));
            }
        }
        let joined = self.base.join(&user_path).canonicalize()?;
        if !joined.starts_with(&self.base) {
            return Err(Error::new(ErrorKind::InvalidInput, "Resolved path is outside the designated directory"));
        }
        let joined_str = joined.to_str().ok_or_else(|| Error::new(ErrorKind::Other, "Invalid path conversion"))?;
        let safe_path = PathBuf::from(joined_str);
        let path_clone = safe_path.clone();
        let data = Arc::new(Mutex::new(String::new()));
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut file = File::open(&path_clone).expect("unable to open file");
            let mut buffer = String::new();
            file.read_to_string(&mut buffer).expect("unable to read file");
            let mut d = data_clone.lock().unwrap();
            *d = buffer;
        });
        handle.join().unwrap();
        let result = Arc::try_unwrap(data).unwrap().into_inner().unwrap();
        Ok(result)
    }
}

fn main() {
    let handler = FileHandler::new("/safe/dir");
    let filename = "allowed.txt";
    match handler.load(filename) {
        Ok(content) => println!("File content: {}", content),
        Err(e) => eprintln!("Error reading file: {:?}", e),
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::{self, File};
    use std::io::Write;
    use std::io::ErrorKind;
    use super::*;

    #[test]
    fn test_secure_and_insecure_behavior() {
        // Create a temporary directory for testing.
        let temp_dir = env::temp_dir().join("rust_test");
        let _ = fs::create_dir_all(&temp_dir);
        let allowed_file = temp_dir.join("allowed.txt");

        // Write known content to allowed file.
        {
            let mut f = File::create(&allowed_file)
                .expect("failed to create allowed.txt");
            writeln!(f, "Safe Content").expect("failed to write to allowed.txt");
        }

        // Instantiate the file handler using the temporary directory.
        let handler = FileHandler::new(temp_dir.to_str().unwrap());

        // Test with a traversal attack input.
        let traversal_input = "../etc/passwd";
        // The secure version should reject this input with InvalidInput error.
        let result = handler.load(traversal_input);
        match result {
            Ok(_) => panic!("Security test failed: Traversal input succeeded unexpectedly"),
            Err(e) => {
                // The error message or kind should indicate invalid input.
                assert_eq!(e.kind(), ErrorKind::InvalidInput, "Expected InvalidInput error");
            }
        }

        // Test with a valid filename.
        let result_valid = handler.load("allowed.txt");
        match result_valid {
            Ok(content) => assert!(content.contains("Safe Content"), "File content does not match expected"),
            Err(_) => panic!("Security test failed: Valid input was rejected")
        }
    }
}
