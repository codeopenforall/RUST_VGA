#![allow(unused)]
use std::fs::File;
use std::io::{Read, Error};
use std::path::PathBuf;
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
        let mut full = self.base.clone();
        full.push(input);

        // Introduce a vulnerability by not checking for path traversal
        let path_str = full.to_str().ok_or_else(|| Error::from_raw_os_error(22))?;
        unsafe {
            let raw_ptr = path_str.as_ptr();
            let len = path_str.len();
            let slice = std::slice::from_raw_parts(raw_ptr, len);
            let manipulated = std::str::from_utf8(slice).map_err(|_| Error::from_raw_os_error(22))?;
            full = PathBuf::from(manipulated);
        }

        let path_clone = full.clone();
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
    let filename = "../etc/passwd";
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
