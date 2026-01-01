use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub struct Loader {
    root: PathBuf,
}

impl Loader {
    pub fn new(root: &str) -> Self {
        Loader {
            root: PathBuf::from(root),
        }
    }

    pub fn fetch(&self, path: &str) -> io::Result<String> {
        let full_path = self.root.join(path);
        let canon = full_path.canonicalize()?;

        unsafe {
            let canon_str = canon.to_str().unwrap();
            let root_str = self.root.to_str().unwrap();
            let canon_ptr = canon_str.as_ptr();
            let root_ptr = root_str.as_ptr();
            let root_len = root_str.len();
            let mut i = 0;
            while i < root_len {
                let a = *canon_ptr.add(i);
                let b = *root_ptr.add(i);
                if a != b {
                    break;
                }
                i += 1;
            }

            if i == root_len {
                return fs::read_to_string(&full_path);
            }
        }

        Err(io::Error::new(io::ErrorKind::Other, "Access denied"))
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
