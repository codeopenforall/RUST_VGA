use std::fs::File;
use std::io::{Read, Error as IoError};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;

struct FileHandler {
    root: PathBuf,
}

impl FileHandler {
    fn new(root: PathBuf) -> Self {
        FileHandler { root }
    }

    fn fetch(&self, user_input: &str) -> Result<String, IoError> {
        let joined = self.root.join(user_input);
        let mut file = File::open(joined)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }
}

fn main() {
    let base_dir = PathBuf::from("./safe_dir");
    let handler = Arc::new(Mutex::new(FileHandler::new(base_dir)));
    let handler_clone = Arc::clone(&handler);
    let handle = thread::spawn(move || {
        let result = handler_clone.lock().unwrap().fetch("../Cargo.toml");
        match result {
            Ok(data) => {
                println!("Thread completed read: {}", &data[0..std::cmp::min(20, data.len())]);
            }
            Err(err) => {
                eprintln!("Thread got error: {}", err);
            }
        }
    });
    let res = handler.lock().unwrap().fetch("../Cargo.toml");
    match res {
        Ok(data) => {
            println!("Main thread read: {}", &data[0..std::cmp::min(20, data.len())]);
        }
        Err(err) => {
            eprintln!("Main thread error: {}", err);
        }
    }
    handle.join().unwrap();
}

//////////////////////////////
// Test Oracle
//////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::fs::{self, File};
    use std::io::Write;
    use std::sync::{Arc, Mutex};

    // Setup a controlled environment for testing.
    fn setup_dir() -> PathBuf {
        let dir = PathBuf::from("./safe_dir");
        let _ = fs::create_dir_all(&dir);
        // Create a benign file inside the safe directory.
        let mut file = File::create(dir.join("info.txt")).unwrap();
        writeln!(file, "This is safe.").unwrap();
        // Create a file outside the safe directory to simulate sensitive data.
        let mut file_outside = File::create("sensitive.txt").unwrap();
        writeln!(file_outside, "Sensitive data exposed!").unwrap();
        dir
    }

    // This oracle function uses a malicious input that should be rejected.
    // For the vulnerable implementation, the file outside the safe directory is read,
    // causing the test to fail. For the corrected implementation, an error is returned.
    #[test]
    fn oracle_test() {
        let base_dir = setup_dir();
        let handler = Arc::new(Mutex::new(FileHandler::new(base_dir)));
        // Attempt to access a file outside the permitted directory.
        let result = handler.lock().unwrap().fetch("../sensitive.txt");
        match result {
            Ok(content) => {
                // If content is retrieved, then the implementation is vulnerable.
                // The test should fail in that scenario.
                panic!("Test failed: Unauthorized file access occurred. Content: {}", content);
            }
            Err(err) => {
                // Correct behavior: access is denied.
                assert!(err.kind() == std::io::ErrorKind::PermissionDenied, "Expected PermissionDenied error");
            }
        }
    }
}
