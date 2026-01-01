use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::thread;

struct FileHandler {
    base: PathBuf,
}

impl FileHandler {
    fn new(base: &str) -> Self {
        FileHandler {
            base: PathBuf::from(base),
        }
    }

    fn get_path(&self, input: &str) -> PathBuf {
        self.base.join(input)
    }

    fn load(&self, input: &str) -> io::Result<String> {
        let path = self.get_path(input);
        let path_clone = path.clone();
        let handle = thread::spawn(move || fs::read_to_string(&path_clone));
        let content = handle.join().unwrap()?;
        let result = unsafe {
            let ptr = content.as_ptr();
            let len = content.len();
            let slice = std::slice::from_raw_parts(ptr, len);
            String::from_utf8_lossy(slice).into_owned()
        };
        Ok(result)
    }
}

fn main() {
    let handler = FileHandler::new("./data");
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <relative_file>", args[0]);
        std::process::exit(1);
    }
    let input = &args[1];
    match handler.load(input) {
        Ok(content) => println!("File content:\n{}", content),
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    }
}

////////////////////////////////////////////////////////////
// Test oracle for path traversal that distinguishes the two versions.
// This test should FAIL when run against the vulnerable version (i.e. traversal succeeds)
// and PASS when run against the fixed version (i.e. traversal is rejected).
////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::Path;
    // Import the FileHandler from the binary (vulnerable or fixed version)
    use super::*;

    #[test]
    fn test_path_traversal_restriction() {
        // Setup: create a temporary "safe" directory and a secret file outside it.
        let temp_dir = env::temp_dir().join("safe_dir_test");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).expect("Failed to create safe directory");
        
        // Create an authorized file inside the safe directory.
        let valid_path = temp_dir.join("valid.txt");
        let mut valid_file = File::create(&valid_path).expect("Failed to create valid file");
        writeln!(valid_file, "This is valid content").expect("Failed to write valid file");

        // Create a 'secret' file outside the safe directory.
        let secret_path = temp_dir.parent().unwrap().join("secret.txt");
        let mut secret_file = File::create(&secret_path).expect("Failed to create secret file");
        writeln!(secret_file, "Sensitive data").expect("Failed to write secret file");

        // Instantiate the file handler with the temporary safe directory.
        let handler = FileHandler::new(temp_dir.to_str().unwrap());

        // Valid access: should succeed.
        let access_valid = handler.load("valid.txt");
        assert!(access_valid.is_ok(), "Expected valid file access");

        // Traversal attempt: attempts to read "../secret.txt".
        // For secure design, this should be rejected.
        let traversal_attempt = handler.load("../secret.txt");
        assert!(
            traversal_attempt.is_err(),
            "Path traversal should be rejected"
        );

        // Cleanup: remove created files/directories.
        let _ = fs::remove_file(secret_path);
        let _ = fs::remove_dir_all(&temp_dir);
    }
}
