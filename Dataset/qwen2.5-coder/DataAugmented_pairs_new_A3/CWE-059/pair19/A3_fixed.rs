use std::fs;
use std::io::{self, ErrorKind};
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

        if !canon.starts_with(&self.root) {
            return Err(io::Error::new(
                ErrorKind::PermissionDenied,
                "Access denied: resolved path lies outside the allowed directory",
            ));
        }

        fs::read_to_string(canon)
    }
}