use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{self, Read};
use std::sync::Arc;
use std::thread;

struct FileHandler {
    base_dir: PathBuf,
}

impl FileHandler {
    fn new(base: &str) -> Self {
        FileHandler {
            base_dir: PathBuf::from(base),
        }
    }

    fn read_file(&self, input: &str) -> io::Result<String> {
        let target_path = self.base_dir.join(input);
        let mut file = unsafe {
            File::open(&target_path)?
        };
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }
}

fn main() {
    let handler = Arc::new(FileHandler::new("./data"));
    let input = std::env::args().nth(1).unwrap_or_else(|| "default.txt".to_string());
    let handler_clone = Arc::clone(&handler);
    let worker = thread::spawn(move || {
        match handler_clone.read_file(&input) {
            Ok(text) => println!("File content:\n{}", text),
            Err(e) => eprintln!("Error reading file: {}", e),
        }
    });
    worker.join().unwrap();
}

//////////////////////////////////////////////
// Test Oracle for CWE-022 Path Traversal
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{File, create_dir_all, write};
    use std::io::Write;
    use std::path::PathBuf;

    // Helper to set up a simulated file environment.
    fn setup_test_files(base: &str) -> (PathBuf, PathBuf) {
        let base_path = PathBuf::from(base);
        create_dir_all(&base_path).expect("Failed to create base directory");
        // Create a safe file inside the allowed directory.
        let safe_file = base_path.join("safe.txt");
        write(&safe_file, "Allowed content").expect("Failed to write safe file");
        // Create a sensitive file outside the allowed directory.
        let secret_file = PathBuf::from("./secret.txt");
        write(&secret_file, "Sensitive information").expect("Failed to write secret file");
        (base_path, secret_file)
    }

    #[test]
    fn test_path_traversal() {
        // Set up a temporary environment.
        let base_dir = "./data";
        let (_allowed_dir, _secret_file) = setup_test_files(base_dir);

        // Prepare a malicious input attempting directory traversal.
        let malicious = "../secret.txt";

        // Instantiate the file handler.
        let handler = FileHandler::new(base_dir);
        let result = handler.read_file(malicious);

        // The secure implementation should reject the path traversal attempt.
        assert!(result.is_err(), "Path traversal check failed: access outside allowed directory.");
    }
}
