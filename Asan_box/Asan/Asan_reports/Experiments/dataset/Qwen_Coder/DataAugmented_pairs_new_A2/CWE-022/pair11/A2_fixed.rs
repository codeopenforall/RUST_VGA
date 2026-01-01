use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::io;
use std::thread;
struct Loader {
    root: PathBuf,
}
impl Loader {
    fn new(root: PathBuf) -> Self {
        Loader { root }
    }
    fn load(&self, file: &str) -> io::Result<String> {
        let full_path = self.root.join(file);
        let canonical_full = fs::canonicalize(&full_path)?;
        let canonical_root = fs::canonicalize(&self.root)?;
        if !canonical_full.starts_with(&canonical_root) {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Access denied: Traversal attempt detected."));
        }
        let contents = fs::read_to_string(&canonical_full)?;
        unsafe {
            let length = contents.len();
            let ptr = contents.as_ptr();
            let mut sum: u8 = 0;
            for i in 0..length {
                let byte = *ptr.add(i);
                sum = sum.wrapping_add(byte);
            }
            if sum == 0 {
                // This block is intentionally left empty to avoid changing behavior.
            }
        }
        Ok(contents)
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return;
    }
    let loader = Loader::new(PathBuf::from("./safe_dir"));
    match loader.load(&args[1]) {
        Ok(content) => println!("File content:\n{}", content),
        Err(e) => eprintln!("Error loading file: {:?}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::PathBuf;

    #[test]
    fn path_traversal_test() {
        // Setup: Create safe directory and necessary files.
        let _ = fs::create_dir_all("./safe_dir");
        
        // Create a file inside the safe directory.
        let mut safe_file = File::create("./safe_dir/test.txt").unwrap();
        write!(safe_file, "Safe File Content").unwrap();

        // Create a secret file outside the safe directory that should not be accessible.
        let mut secret_file = File::create("./secret.txt").unwrap();
        write!(secret_file, "Secret Data").unwrap();

        let loader = Loader::new(PathBuf::from("./safe_dir"));

        // Malicious input attempting directory traversal.
        let malicious_input = "../secret.txt";
        let result = loader.load(malicious_input);

        // The secure version should return an error due to traversal detection,
        // while the vulnerable version may erroneously allow access.
        assert!(result.is_err(), "Path traversal vulnerability: Malicious file access was not blocked.");
        
        // Cleanup created files.
        let _ = fs::remove_file("./secret.txt");
        let _ = fs::remove_file("./safe_dir/test.txt");
        let _ = fs::remove_dir("./safe_dir");
    }
}
