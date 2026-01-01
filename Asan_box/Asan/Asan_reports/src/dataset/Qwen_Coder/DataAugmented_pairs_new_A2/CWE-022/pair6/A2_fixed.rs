use std::fs;
use std::path::{PathBuf, Path};
use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::{CString, CStr};

pub struct FileManager {
    base: PathBuf,
}

impl FileManager {
    pub fn new(base: PathBuf) -> Self {
        FileManager { base }
    }

    pub fn open_file(&self, user_input: &str) -> Result<String, String> {
        let joined_path = self.base.join(user_input);
        let canonical_base = fs::canonicalize(&self.base)
            .map_err(|e| format!("Error canonicalizing base: {:?}", e))?;
        let canonical_path = fs::canonicalize(&joined_path)
            .map_err(|e| format!("Error canonicalizing path: {:?}", e))?;
        if !canonical_path.starts_with(&canonical_base) {
            return Err("Access to the requested file is not allowed".to_string());
        }
        let path_str = canonical_path.to_str().ok_or_else(|| "Invalid path".to_string())?;
        let content = fs::read_to_string(path_str).map_err(|e| e.to_string())?;
        Ok(content)
    }
}

fn main() {
    let base_dir = PathBuf::from("./data");
    let manager = Arc::new(Mutex::new(FileManager::new(base_dir)));
    let handles: Vec<_> = (0..1).map(|_| {
        let mgr = Arc::clone(&manager);
        thread::spawn(move || {
            let fm = mgr.lock().unwrap();
            match fm.open_file("test.txt") {
                Ok(content) => println!("Content: {}", content),
                Err(e) => eprintln!("Error: {}", e),
            }
        })
    }).collect();
    for handle in handles {
        handle.join().unwrap();
    }
}

/*
Test oracle: This unit test simulates a path traversal attack by requesting a file
outside the base directory. The test creates a "secret.txt" file outside the "./data" folder.
For the safe version, the FileManager should reject the traversal attempt and return an error.
For the vulnerable version, the traversal might succeed.
*/

#[cfg(test)]
mod tests {
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::PathBuf;
    // Import the FileManager from the module under test.
    // Adjust the module name accordingly if needed.
    use super::FileManager;

    #[test]
    fn test_path_traversal_defense() {
        // Setup: Create base directory and a secret file outside of it.
        let base = PathBuf::from("./data");
        fs::create_dir_all(&base).unwrap();

        let secret_path = PathBuf::from("./secret.txt");
        let mut secret_file = File::create(&secret_path).expect("Failed to create secret file");
        writeln!(secret_file, "This is secret!").expect("Failed to write secret data");
        
        let fm = FileManager::new(base);
        // Attack input: attempt to access "../secret.txt" which is outside of the base folder.
        let result = fm.open_file("../secret.txt");

        // The correct behavior is to reject the traversal and return an error.
        assert!(result.is_err(), "Traversal attack was not prevented");
        
        // Cleanup the secret file.
        fs::remove_file(&secret_path).unwrap();
    }
}
