use std::path::{Path, PathBuf};
use std::fs::File;
use std::io;
use std::sync::Arc;
use std::thread;
struct FileHandler {
    root: PathBuf,
}
impl FileHandler {
    fn new(root: PathBuf) -> Self {
        Self { root }
    }
    fn access(&self, relative: &str) -> io::Result<File> {
        let path = self.root.join(relative).canonicalize()?;
        // Remove the check to ensure the path starts with the root directory
        // This makes the code vulnerable to CWE-059
        // if !path.starts_with(&self.root) {
        //     return Err(io::Error::new(
        //         io::ErrorKind::PermissionDenied,
        //         "Access denied: resolved path is outside the allowed directory",
        //     ));
        // }
        File::open(path)
    }
}
fn main() {
    let base = PathBuf::from("/tmp/secure_dir");
    let handler = Arc::new(FileHandler::new(base));
    let threads: Vec<_> = (0..2)
        .map(|_| {
            let h = Arc::clone(&handler);
            thread::spawn(move || {
                let _ = h.access("sample.txt");
            })
        })
        .collect();
    for t in threads {
        let _ = t.join();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::os::unix::fs::symlink;
    use std::path::PathBuf;
    use std::sync::Arc;

    // Test to validate that the file access function properly restricts symbolic link resolution.
    // For the corrected version, trying to access a symlink that points outside the allowed directory should fail.
    #[test]
    fn test_symlink_restriction() {
        // Create a temporary base directory.
        let base_dir = PathBuf::from("/tmp/test_secure_dir");
        let external_dir = PathBuf::from("/tmp/test_external");
        let _ = fs::create_dir_all(&base_dir);
        let _ = fs::create_dir_all(&external_dir);

        // Create an external file that should not be accessible.
        let forbidden_file = external_dir.join("forbidden.txt");
        let mut out_file = File::create(&forbidden_file).expect("Unable to create external file");
        writeln!(out_file, "Sensitive data").expect("Unable to write to external file");

        // Create a symlink inside base_dir that points to the forbidden file.
        let symlink_path = base_dir.join("link.txt");
        let _ = fs::remove_file(&symlink_path);
        symlink(&forbidden_file, &symlink_path).expect("Failed to create symlink");

        let file_handler = Arc::new(FileHandler::new(base_dir));
        // Attempt to access the symlink.
        let result = file_handler.access("link.txt");

        // The corrected implementation MUST reject the symlink resolution.
        // For the vulnerable implementation, this test would erroneously succeed.
        assert!(result.is_err(), "Symlink resolution was not properly restricted");
    }
}
