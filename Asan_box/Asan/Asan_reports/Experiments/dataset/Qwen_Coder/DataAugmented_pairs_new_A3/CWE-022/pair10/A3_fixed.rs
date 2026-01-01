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

//////////////////////////////
// Test Oracle Function
//////////////////////////////

// This test oracle verifies that requests attempting path traversal are rejected.
// The test fails for the vulnerable implementation (which erroneously permits traversal)
// and passes for the corrected version.

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_path_traversal_defense() {
        // Set up the file server with the safe directory.
        let server = FileServer::new(PathBuf::from("./safe_dir"));
        
        // Attempt to access a file outside the safe directory.
        // In the fixed version, this should return an error.
        let result = server.serve("../Cargo.toml");
        assert!(result.is_err(), "Traversal attempt was not blocked");
    }
}
