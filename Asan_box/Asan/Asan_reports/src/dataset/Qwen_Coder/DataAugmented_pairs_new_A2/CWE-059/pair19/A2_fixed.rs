use std::fs;
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

struct Loader {
    root: PathBuf,
    cache: Arc<Mutex<Vec<(PathBuf, String)>>>,
}

impl Loader {
    pub fn new(base: &str) -> Self {
        Self {
            root: PathBuf::from(base),
            cache: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn fetch(&self, file: &str) -> io::Result<String> {
        let path = self.root.join(file);
        let canon = fs::canonicalize(&path)?;
        if !canon.starts_with(&self.root) {
            return Err(io::Error::new(
                ErrorKind::PermissionDenied,
                "Access denied: resolved path lies outside the allowed directory",
            ));
        }
        let content = fs::read_to_string(&canon)?;
        let mut cache_lock = self.cache.lock().unwrap();
        cache_lock.push((canon, content.clone()));
        Ok(content)
    }
}

fn main() {
    let loader = Loader::new("safe_dir");
    match loader.fetch("test.txt") {
        Ok(data) => println!("Loaded content:\n{}", data),
        Err(e) => eprintln!("Error encountered: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::ErrorKind;
    use std::path::Path;

    // This helper function sets up a controlled file system structure:
    // - Creates a "safe_dir" with a legitimate file.
    // - Creates an external file ("secret.txt").
    // - Creates a symlink inside "safe_dir" (named "link.txt") that points to "../secret.txt".
    // The test expects that fetching "link.txt" should result in an error in the secure version.
    fn setup_files() {
        // Create safe_dir and files.
        let _ = fs::create_dir("safe_dir");
        fs::write("safe_dir/test.txt", "This is a safe file").unwrap();
        fs::write("secret.txt", "Sensitive secret content").unwrap();

        // Remove the symlink if it exists.
        let link_path = Path::new("safe_dir/link.txt");
        if link_path.exists() {
            let _ = fs::remove_file(link_path);
        }
        // Create a symbolic link depending on the platform.
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink("../secret.txt", "safe_dir/link.txt").unwrap();
        }
        #[cfg(windows)]
        {
            std::os::windows::fs::symlink_file("../secret.txt", "safe_dir/link.txt").unwrap();
        }
    }

    // Oracle test: For a given Loader instance, fetching the symlink "link.txt"
    // should fail in the secure (fixed) version, while the vulnerable version would erroneously succeed.
    #[test]
    fn test_oracle() {
        setup_files();
        let loader = Loader::new("safe_dir");
        let result = loader.fetch("link.txt");
        // The fixed version must return a permission error.
        assert!(result.is_err(), "Security check failed: symlink allowed access");
        if let Err(e) = result {
            assert_eq!(e.kind(), ErrorKind::PermissionDenied, "Unexpected error kind");
        }
    }
}
