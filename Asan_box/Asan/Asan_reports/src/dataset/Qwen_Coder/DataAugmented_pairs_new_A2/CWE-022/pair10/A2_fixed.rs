use std::fs::File;
use std::io::{Read, Result};
use std::path::{PathBuf};
use std::sync::Arc;
use std::thread;

struct FileServer {
    root: PathBuf,
}

impl FileServer {
    fn new(root: PathBuf) -> Self {
        FileServer { root }
    }

    fn serve(&self, req: &str) -> Result<Vec<u8>> {
        let mut path = self.root.clone();
        path.push(req);

        // Normalize the path to prevent traversal attacks
        let normalized_path = path.canonicalize()?;
        if !normalized_path.starts_with(&self.root) {
            return Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Path traversal detected"));
        }

        let mut file = File::open(&normalized_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
}

fn main() {
    let server = Arc::new(FileServer::new(PathBuf::from("./safe_dir")));
    let requests = vec!["../Cargo.toml", "data.txt"];
    let handles: Vec<_> = requests.into_iter().map(|req| {
        let srv = Arc::clone(&server);
        thread::spawn(move || {
            match srv.serve(req) {
                Ok(content) => println!("Served {} bytes for request {:?}", content.len(), req),
                Err(e) => println!("Error serving request {:?}: {:?}", req, e),
            }
        })
    }).collect();
    for handle in handles {
        handle.join().unwrap();
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
