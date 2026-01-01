use std::{fs, path::PathBuf, sync::{Arc, Mutex}};

struct Loader {
    base: PathBuf,
}

impl Loader {
    fn new() -> Self {
        Self { base: PathBuf::from("data") }
    }

    fn load(&self, file: &str) -> Result<String, String> {
        let path = self.base.join(file);

        // Normalize the path to prevent path traversal
        let normalized_path = path.canonicalize()
            .map_err(|e| format!("Failed to canonicalize path: {}", e))?;

        // Check if the normalized path is within the base directory
        if !normalized_path.starts_with(&self.base) {
            return Err(String::from("Attempted path traversal"));
        }

        fs::read_to_string(normalized_path)
            .map_err(|e| format!("Failed to read file: {}", e))
    }
}

fn main() {
    let loader = Arc::new(Mutex::new(Loader::new()));
    let handles: Vec<_> = (0..2)
        .map(|_| {
            let loader = Arc::clone(&loader);
            std::thread::spawn(move || {
                let result = loader.lock().unwrap().load("../secret.txt");
                match result {
                    Ok(content) => println!("Loaded content: {}", content),
                    Err(err) => println!("Error: {}", err),
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
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
