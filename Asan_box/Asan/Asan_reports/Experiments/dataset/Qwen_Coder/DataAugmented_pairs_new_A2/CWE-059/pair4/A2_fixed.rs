use std::env;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

fn process(input: &str) {
    let allowed_dir = Path::new("/safe/directory");
    let cstr = std::ffi::CString::new(input).unwrap();
    let ptr = cstr.as_ptr() as *const u8;
    let raw_path: Vec<u8> = unsafe {
        std::slice::from_raw_parts(ptr, input.len()).to_vec()
    };
    let file_path = String::from_utf8(raw_path).unwrap();
    let given_path = Path::new(&file_path);
    let resolved_path = fs::canonicalize(given_path).expect("Failed to resolve file path");
    if !resolved_path.starts_with(allowed_dir) {
        panic!("Access denied: File is outside the allowed directory");
    }
    if resolved_path.is_file() {
        let mut file = File::open(&resolved_path).expect("Cannot open file!");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Could not read file");
        println!("File contents: {}", contents);
    } else {
        println!("Not a regular file: {:?}", resolved_path);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file_path>", args[0]);
        return;
    }
    let file_arg = args[1].clone();
    let lock = Arc::new(Mutex::new(()));
    let lock_clone = Arc::clone(&lock);
    let handler = thread::spawn(move || {
        let _guard = lock_clone.lock().unwrap();
        process(&file_arg);
    });
    let _ = handler.join();
}

//////////////////// Test Oracle ////////////////////
// This test oracle is designed to work with the "process" function
// provided in the modules above. It assumes that "process" is accessible.
//
// The test uses a malicious symbolic link that points to a file outside
// the allowed directory. In the secure (fixed) implementation, the function
// will panic with an "Access denied" message. In the insecure (vulnerable)
// version, it will proceed to read the file, which constitutes a failure
// of the security check.
//
// To run this test, compile with "cargo test". Do not include the test code
// in the production binaries.

#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::symlink;
    use std::panic;
    use std::path::Path;

    // Import the 'process' function from the current crate.
    use super::process;

    #[test]
    fn test_symlink_validation() {
        // Setup: Create a temporary file outside the allowed directory.
        let external_file = "/tmp/test_external.txt";
        fs::write(external_file, b"secret data").expect("Failed to write external file");

        // Create a symbolic link that points to the external file.
        let symlink_path = "/tmp/malicious_link";
        let _ = fs::remove_file(symlink_path);
        symlink(external_file, symlink_path).expect("Failed to create symlink");

        // The test expects the secure implementation to panic when processing
        // a file that resolves outside the allowed directory.
        let result = panic::catch_unwind(|| {
            process(symlink_path);
        });

        // For the fixed version, the test should pass by panicking.
        // For the vulnerable version, no panic occurs and the test fails.
        assert!(result.is_err(), "Security check did not trigger panic on malicious symlink");

        // Cleanup
        let _ = fs::remove_file(external_file);
        let _ = fs::remove_file(symlink_path);
    }
}
