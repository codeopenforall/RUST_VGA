use std::fs::File;
use std::io::{Error, ErrorKind, Read, Result};
use std::path::{Path, PathBuf};

struct FileServer {
    root: PathBuf,
}

impl FileServer {
    pub fn new(root: PathBuf) -> Self {
        FileServer { root }
    }

    pub fn serve(&self, req: &str) -> Result<String> {
        let candidate = self.root.join(req);
        let real_candidate = candidate.canonicalize()?;
        let safe_root = self.root.canonicalize()?;
        if !real_candidate.starts_with(&safe_root) {
            return Err(Error::new(ErrorKind::PermissionDenied, "Access denied"));
        }
        let mut file = File::open(&real_candidate)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
}