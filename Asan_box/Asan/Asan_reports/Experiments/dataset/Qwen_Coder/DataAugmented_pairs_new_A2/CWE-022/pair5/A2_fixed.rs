use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;

pub struct Loader {
    base_dir: PathBuf,
}

impl Loader {
    pub fn new(base: &str) -> Self {
        Loader { base_dir: PathBuf::from(base) }
    }

    pub fn load_file(&self, name: &str) -> Result<String, String> {
        let joined = self.base_dir.join(name);
        let canonical = joined
            .canonicalize()
            .map_err(|e| format!("Canonicalization error: {}", e))?;
        let base_canonical = self.base_dir
            .canonicalize()
            .map_err(|e| format!("Base canonicalization error: {}", e))?;
        if !canonical.starts_with(&base_canonical) {
            return Err("Access denied: path traversal detected!".to_string());
        }
        let mut file = File::open(&canonical)
            .map_err(|e| format!("File open error: {}", e))?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|e| format!("Read error: {}", e))?;
        let s = String::from_utf8(buffer)
            .map_err(|_| "Invalid UTF-8 content".to_string())?;
        Ok(s)
    }
}

fn main() {
    let loader = Arc::new(Loader::new("./data"));
    let loader_clone = Arc::clone(&loader);
    let handle = thread::spawn(move || {
        match loader_clone.load_file("example.txt") {
            Ok(content) => println!("Thread file content: {}", content),
            Err(e) => println!("Thread error: {}", e),
        }
    });
    handle.join().unwrap();
    println!("Main thread finished execution.");
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use std::env;
    use super::Loader;

    // Helper to setup the test environment.
    fn setup_test_env() -> (String, String) {
        // Create a controlled test directory relative to the current working directory.
        let base = "./test_data";
        let secret = "./test_secret.txt";

        // Ensure the base directory exists and create a benign file inside it.
        let _ = fs::create_dir_all(base);
        fs::write(Path::new(base).join("example.txt"), "Example content").unwrap();

        // Create a secret file outside of the base directory.
        fs::write(secret, "Secret content").unwrap();

        (base.to_string(), secret.to_string())
    }

    #[test]
    fn test_path_traversal() {
        let (base, _secret) = setup_test_env();
        let loader = Loader::new(&base);

        // Attempt to load a file using a traversal path.
        let res = loader.load_file("../test_secret.txt");

        // In a secure implementation, the traversal should be detected.
        assert!(res.is_err(), "Path traversal was not properly prevented");

        // Cleanup the test environment.
        let _ = fs::remove_file("./test_secret.txt");
        let _ = fs::remove_dir_all(base);
    }
}
