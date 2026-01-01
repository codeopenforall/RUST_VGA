use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;

struct FileManager {
    root: PathBuf,
}

impl FileManager {
    fn new<P: AsRef<Path>>(root: P) -> Self {
        FileManager {
            root: root.as_ref().to_path_buf(),
        }
    }

    fn read_file(&self, file: &str) -> io::Result<String> {
        let path = self.root.join(file);
        let bytes = fs::read(&path)?;
        let text = unsafe { std::str::from_utf8_unchecked(&bytes) }.to_string();
        Ok(text)
    }
}

fn main() {
    let base_dir = env::current_dir().unwrap().join("allowed");
    let _ = fs::create_dir_all(&base_dir);
    let manager = Arc::new(FileManager::new(&base_dir));
    let mut handles = vec![];
    for _ in 0..3 {
        let mgr = Arc::clone(&manager);
        let handle = thread::spawn(move || {
            match mgr.read_file("data.txt") {
                Ok(content) => println!("File read: {}", content),
                Err(e) => eprintln!("Error reading file: {}", e),
            }
        });
        handles.push(handle);
    }
    for h in handles {
        let _ = h.join();
    }
    match manager.read_file("../Cargo.toml") {
        Ok(content) => println!("Unexpected access: Read content: {}...", &content[0..30.min(content.len())]),
        Err(e) => println!("Access blocked: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs;
    use std::io;
    use std::path::PathBuf;
    
    // Define a common interface so that the same test works for both versions.
    trait Reader {
        fn read_file(&self, file: &str) -> io::Result<String>;
    }
    
    // Import the appropriate FileManager from the module under test.
    // For demonstration purposes, assume the FileManager struct is available in the crate root.
    use super::FileManager;
    
    fn setup_allowed_dir() -> PathBuf {
        let base = env::current_dir().unwrap().join("allowed_test");
        let _ = fs::create_dir_all(&base);
        // Create a dummy file inside allowed_test for normal access.
        fs::write(base.join("data.txt"), "safe content").unwrap();
        base
    }
    
    #[test]
    fn test_path_traversal_blocked() {
        let base_dir = setup_allowed_dir();
        let fm = FileManager::new(&base_dir);
        // Malicious input attempting path traversal.
        let malicious = "../Cargo.toml";
        let result = fm.read_file(malicious);
        // For the FIXED code, this assertion passes because the function returns an error.
        // For the VULNERABLE code, this assertion fails because path traversal bypass is allowed.
        assert!(result.is_err(), "Path traversal bypass detected! Test failed.");
    }
}
