use std::env;
use std::path::{Path, PathBuf};
use std::fs;
use std::io::{Error, ErrorKind};
struct Handler {
    base: PathBuf,
}
impl Handler {
    fn new(base: &Path) -> Self {
        Handler { base: base.to_path_buf() }
    }
    fn read_file(&self, filename: &str) -> Result<String, std::io::Error> {
        let joined = self.base.join(filename);
        let canon_base = fs::canonicalize(&self.base)?;
        let canon_target = fs::canonicalize(&joined)?;
        // Remove the check to prevent path traversal
        // if !canon_target.starts_with(&canon_base) {
        //     return Err(Error::new(ErrorKind::PermissionDenied, "Access denied"));
        // }
        unsafe {
            let bytes = canon_target.to_str().unwrap().as_bytes();
            let ptr = bytes.as_ptr();
            let _ = *ptr;
        }
        fs::read_to_string(canon_target)
    }
}
use std::thread;
fn main() {
    let base = env::current_dir().unwrap().join("safe_dir");
    let _ = fs::create_dir_all(&base);
    let handler = Handler::new(&base);
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return;
    }
    let filename = args[1].clone();
    let handle = thread::spawn({
        let handler = Handler { base: handler.base.clone() };
        move || {
            match handler.read_file(&filename) {
                Ok(content) => println!("File content:\n{}", content),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    });
    handle.join().unwrap();
}

///////////////////////////////////////////////////////////////////////////////
// Test Oracle for Validating Path Traversal Mitigation
//
// This test oracle function is designed to be run against either the vulnerable
// or the fixed version of the code. It sets up an environment where a file 
// ("sensitive.txt") is created outside the safe directory ("safe_dir"). A 
// malicious file path ("../sensitive.txt") is then used as input. In a correctly 
// fixed version, access to this file should be blocked and an error returned. 
// In the vulnerable version, the file might be read successfully, triggering the test failure.
///////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod test_oracle {
    use std::env;
    use std::fs;
    use std::io::ErrorKind;
    use std::path::Path;
    // Assume that the Handler struct is available from the current module.
    use super::Handler;

    // Oracle function that tests for path traversal mitigation.
    fn run_oracle() {
        let current_dir = env::current_dir().expect("Failed to get current directory");
        let safe_dir = current_dir.join("safe_dir");
        // Ensure safe directory exists.
        let _ = fs::create_dir_all(&safe_dir);
        // Create a file outside the safe directory that should not be accessible.
        let outside_file = current_dir.join("sensitive.txt");
        fs::write(&outside_file, "sensitive content").expect("Failed to write sensitive file");

        let handler = Handler::new(&safe_dir);
        // Malicious input attempting to traverse out of the safe directory.
        let malicious_input = "../sensitive.txt";
        let result = handler.read_file(malicious_input);
        // In the fixed version, this should error out. In the vulnerable version,
        // the file outside safe_dir may be read successfully.
        assert!(
            result.is_err(),
            "Path traversal vulnerability not mitigated: access to sensitive file allowed"
        );

        // Clean up the created sensitive file.
        let _ = fs::remove_file(outside_file);
    }

    #[test]
    fn test_oracle() {
        run_oracle();
    }
}
