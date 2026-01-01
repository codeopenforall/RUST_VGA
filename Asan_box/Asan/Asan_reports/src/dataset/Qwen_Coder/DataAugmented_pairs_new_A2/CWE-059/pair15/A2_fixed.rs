use std::fs;
use std::ffi::{CString, CStr};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;

pub struct FileManager {
    base: PathBuf,
}

impl FileManager {
    pub fn new(base: &Path) -> Self {
        Self {
            base: base.to_path_buf(),
        }
    }

    pub fn open_file(&self, name: &str) -> std::io::Result<String> {
        let mut full_path = self.base.join(name);
        if full_path.exists() {
            unsafe {
                let c_str = CString::new(full_path.to_string_lossy().as_bytes()).unwrap();
                let ptr = c_str.as_ptr();
                let recovered = CStr::from_ptr(ptr).to_string_lossy().into_owned();
                full_path = PathBuf::from(recovered);
            }
        }
        let canonical_base = fs::canonicalize(&self.base)?;
        let canonical_full = fs::canonicalize(&full_path)?;
        if !canonical_full.starts_with(&canonical_base) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "Access outside allowed directory denied",
            ));
        }
        fs::read_to_string(&full_path)
    }
}

fn main() {
    let base = Path::new("./safe_dir");
    let manager = FileManager::new(&base);
    let shared_manager = Arc::new(manager);
    let handle_manager = Arc::clone(&shared_manager);
    let handle = thread::spawn(move || {
        let result = handle_manager.open_file("../etc/passwd");
        match result {
            Ok(content) => println!("File content: {}", &content[..std::cmp::min(content.len(), 100)]),
            Err(e) => eprintln!("Error reading file: {}", e),
        }
    });
    handle.join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::os::unix::fs::symlink;
    use std::path::Path;

    #[test]
    fn test_symlink_traversal_prevention() {
        // Set up a temporary safe directory.
        let base = Path::new("./safe_dir_test");
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(&base).expect("Failed to create safe directory");

        // Create a sensitive file outside the safe directory.
        fs::write("sensitive.txt", "Top Secret Data").expect("Failed to create sensitive file");

        // Within the safe directory, create a symlink that points outside.
        let link_path = base.join("link");
        let _ = fs::remove_file(&link_path);
        symlink("../sensitive.txt", &link_path).expect("Failed to create symlink");

        let mgr = FileManager::new(base);
        let result = mgr.open_file("link");

        // The fixed implementation should block such traversal.
        // For the vulnerable version this test would erroneously succeed.
        assert!(result.is_err(), "Symlink traversal should be blocked.");
    }
}
