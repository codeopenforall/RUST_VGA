use std::io::{Read, Error as IoError, ErrorKind};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

struct FileHandler {
    root: PathBuf,
}

impl FileHandler {
    fn new(root: PathBuf) -> Self {
        FileHandler { root }
    }

    pub fn fetch(&self, user_input: &str) -> Result<String, IoError> {
        let candidate = self.root.join(user_input);
        let candidate_canon = candidate.canonicalize()?;
        let base_canon = self.root.canonicalize()?;
        if !candidate_canon.starts_with(&base_canon) {
            return Err(IoError::new(
                ErrorKind::PermissionDenied,
                "access denied: outside permitted directory",
            ));
        }

        let mut file = File::open(candidate_canon)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        Ok(data)
    }
}

fn main() {
    // Example usage
    let base_dir = PathBuf::from("./safe_dir");
    let handler = Arc::new(Mutex::new(FileHandler::new(base_dir)));
    let result = handler.lock().unwrap().fetch("info.txt");
    match result {
        Ok(content) => println!("File content: {}", content),
        Err(err) => println!("Error: {}", err),
    }
}