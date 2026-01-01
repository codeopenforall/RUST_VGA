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