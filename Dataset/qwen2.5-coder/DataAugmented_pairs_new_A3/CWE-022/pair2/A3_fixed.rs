use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

pub struct Config {
    base_dir: PathBuf,
}

impl Config {
    pub fn new(base: &str) -> Self {
        let base_path = PathBuf::from(base).canonicalize().expect("Invalid base directory");
        Config {
            base_dir: base_path,
        }
    }

    pub fn read_resource(&self, user_input: &str) -> io::Result<String> {
        let full_path = self.base_dir.join(user_input);
        let canon_full = full_path.canonicalize()?;
        if !canon_full.starts_with(&self.base_dir) {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Access denied"));
        }
        let mut file = File::open(canon_full)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
}