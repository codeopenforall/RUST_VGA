use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;

struct FileHandler {
    base: PathBuf,
}

impl FileHandler {
    fn new(base: &str) -> Self {
        Self { base: PathBuf::from(base) }
    }

    fn get_content(&self, input: &str) -> std::io::Result<String> {
        let path = self.base.join(input);

        // Normalize the path to prevent traversal
        let normalized_path = fs::canonicalize(&path)?;
        let base_normalized = fs::canonicalize(&self.base)?;

        // Check if the normalized path is within the base directory
        if !normalized_path.starts_with(&base_normalized) {
            return Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Path traversal detected"));
        }

        fs::read_to_string(path)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let handler = Arc::new(FileHandler::new("./data"));
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }
    let input = args[1].clone();
    let handler_clone = handler.clone();
    let thread_handle = thread::spawn(move || {
        match handler_clone.get_content(&input) {
            Ok(c) => println!("Content:\n{}", c),
            Err(e) => println!("Error encountered: {}", e),
        }
    });
    let _ = thread_handle.join();
}