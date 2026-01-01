use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;

struct Loader {
    base: String,
}

trait FileAccess {
    fn get_content(&self, rel: &str) -> Result<String, String>;
}

impl FileAccess for Loader {
    fn get_content(&self, rel: &str) -> Result<String, String> {
        let full_path = format!("{}{}", self.base, rel);
        let data = fs::read_to_string(&full_path).map_err(|e| e.to_string())?;
        unsafe {
            let ptr = data.as_ptr();
            let len = data.len();
            let slice = std::slice::from_raw_parts(ptr, len);
            Ok(String::from_utf8_lossy(slice).into_owned())
        }
    }
}

fn main() {
    let loader = Loader { base: "allowed/".to_string() };
    let shared_results = Arc::new(Mutex::new(Vec::new()));
    let loader_arc = Arc::new(loader);
    let handles: Vec<_> = (0..3)
        .map(|i| {
            let loader_thread = Arc::clone(&loader_arc);
            let results_thread = Arc::clone(&shared_results);
            thread::spawn(move || {
                let path = format!("file{}.txt", i);
                let res = loader_thread.get_content(&path);
                results_thread.lock().unwrap().push(res);
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    let results = shared_results.lock().unwrap();
    for entry in results.iter() {
        match entry {
            Ok(content) => println!("File content: {}", content),
            Err(err) => println!("Error: {}", err),
        }
    }
}

/*
Test Oracle Function

This function serves as a test oracle to verify the protection against path traversal.
It attempts to access a file outside the allowed directory using a relative path such as "../Cargo.toml".
For the vulnerable code, this call would succeed (returning Ok with file content), causing the test to fail.
For the fixed code, the validation check should reject the attempt, returning an Err.
*/

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;

    // Helper function to set up a dummy allowed directory with a safe file.
    fn setup_allowed() {
        let _ = fs::create_dir("allowed");
        let safe_file = "allowed/safe.txt";
        let mut file = File::create(safe_file).expect("Unable to create safe file");
        file.write_all(b"Safe Content").expect("Unable to write to safe file");
    }

    #[test]
    fn test_protection() {
        setup_allowed();

        let loader = Loader { base: "allowed/".to_string() };

        // Attempt to access a file using a path traversal payload.
        // In a secure implementation, this should be rejected.
        let result = loader.get_content("../Cargo.toml");
        assert!(result.is_err(), "Test failed: external file was accessed!");
    }
}
