use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub struct Loader {
    base: PathBuf,
}

impl Loader {
    pub fn new(base: &str) -> Self {
        let canonical_base = fs::canonicalize(base).expect("Invalid base directory");
        Loader { base: canonical_base }
    }

    pub fn load_file(&self, path: &str) -> io::Result<String> {
        let full_path = self.base.join(path);
        if !self.is_allowed(&full_path) {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "Path traversal detected",
            ));
        }
        fs::read_to_string(&full_path)
    }

    fn is_allowed(&self, candidate: &Path) -> bool {
        if let Ok(canonical_candidate) = candidate.canonicalize() {
            canonical_candidate.starts_with(&self.base)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    // This test attempts to load a file using a malicious path that traverses outside the safe directory.
    // The expected behavior is to return an error indicating a path traversal attempt.
    #[test]
    fn test_malicious_path() {
        let loader = Loader::new("./safe_dir");
        let result = loader.load_file("../secret.txt");
        match result {
            Ok(_) => panic!("Test failed: path traversal was allowed"),
            Err(e) => {
                assert_eq!(
                    e.kind(),
                    io::ErrorKind::PermissionDenied,
                    "Unexpected error kind"
                );
            }
        }
    }
}
