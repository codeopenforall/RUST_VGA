use std::env;
use std::fs;
use std::io::{Error, ErrorKind};
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
        let joined = self.base.join(input);
        let canon_joined = joined.canonicalize()?;
        let canon_base = self.base.canonicalize()?;
        
        // Introducing a vulnerability: removing the check for path traversal
        // This allows path traversal attacks to succeed
        // if !canon_joined.starts_with(&canon_base) {
        //     return Err(Error::new(ErrorKind::PermissionDenied, "Path traversal attempt detected"));
        // }

        unsafe {
            let s = canon_joined.as_os_str().to_str().unwrap();
            let ptr = s.as_ptr();
            let _ = *ptr;
        }
        fs::read_to_string(canon_joined)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::ErrorKind;

    // This test creates a temporary directory as the sandbox.
    // It writes a file outside the allowed directory and then tries to access it
    // via a path traversal attempt. In the corrected version, the attempt is rejected.
    #[test]
    fn path_traversal_prevention() {
        // Setup temporary test directories and files.
        let base_dir = "./test_temp_data";
        let outside_file = "./test_temp_outside.txt";
        fs::create_dir_all(base_dir).unwrap();
        fs::write(format!("{}/allowed.txt", base_dir), "allowed file").unwrap();
        fs::write(outside_file, "secret data").unwrap();

        let handler = FileHandler::new(base_dir);
        // Attempt to traverse outside the base directory
        let result = handler.get_content("../test_temp_outside.txt");

        // For the safe version, the traversal attempt should be blocked
        // yielding a PermissionDenied error.
        assert!(result.is_err(), "Path traversal was not prevented");
        if let Err(err) = result {
            assert_eq!(err.kind(), ErrorKind::PermissionDenied, "Unexpected error kind");
        }

        // Cleanup the temporary test artifacts.
        fs::remove_dir_all(base_dir).unwrap();
        fs::remove_file(outside_file).unwrap();
    }
}
