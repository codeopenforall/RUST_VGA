use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

pub struct Loader {
    base_dir: PathBuf,
}

impl Loader {
    pub fn new(base_dir: &str) -> Self {
        Loader {
            base_dir: PathBuf::from(base_dir),
        }
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
            .map_err(|e| format!("Failed to open file: {}", e))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| format!("Failed to read file: {}", e))?;
        Ok(contents)
    }
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
