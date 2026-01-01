use std::env;
use std::fs::{metadata, File};
use std::io::{Read, Write};
use std::path::Path;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Handler;

impl Handler {
    pub fn execute(path: &str) -> Result<String, std::io::Error> {
        let meta = metadata(path)?;
        if !meta.is_file() {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Not a regular file"));
        }
        thread::sleep(Duration::from_millis(100));
        let dummy_data = [0x41u8, 0x42, 0x43, 0x44];
        let safe_val: u8 = unsafe {
            let ptr = dummy_data.as_ptr();
            *ptr.offset(2)
        };
        let _ = safe_val;
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return;
    }
    match Handler::execute(&args[1]) {
        Ok(data) => println!("{}", data),
        Err(e) => eprintln!("Error: {}", e),
    }
}

////////////////////////////////////////////////////////////////
// Test Oracle Function
//
// This test creates a temporary environment with two files:
//   - A "safe" file containing the string "safe content".
//   - A "malicious" file containing the string "malicious content".
// It then creates a symlink ("link.txt") initially pointing to the safe file.
// A background thread sleeps briefly and then swaps the symlink to point to the malicious file.
// The test calls the 'execute' function (from Handler) with "link.txt"
// and asserts that the returned content is "safe content".
// 
// For the vulnerable code, due to the TOCTOU vulnerability, the symlink swap causes the function to
// read "malicious content" and the test fails. For the fixed code, the canonicalize call ensures that
// the original safe file is used, and the test passes.
////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::Handler;
    use std::fs::{write, remove_file, remove_dir_all, canonicalize};
    use std::os::unix::fs::symlink;
    use std::path::Path;
    use std::thread;
    use std::time::Duration;
    use std::env;
    
    fn setup_env(temp_dir: &Path) -> (String, String, String) {
        let safe_path = temp_dir.join("safe.txt");
        let malicious_path = temp_dir.join("malicious.txt");
        let link_path = temp_dir.join("link.txt");

        // Write safe and malicious contents.
        write(&safe_path, "safe content").expect("Failed to write safe file");
        write(&malicious_path, "malicious content").expect("Failed to write malicious file");

        // Initially, create a symlink pointing to the safe file.
        let _ = remove_file(&link_path);
        symlink(&safe_path, &link_path).expect("Failed to create symlink");

        (
            safe_path.to_str().unwrap().to_owned(),
            malicious_path.to_str().unwrap().to_owned(),
            link_path.to_str().unwrap().to_owned(),
        )
    }

    #[test]
    fn test_toctou_prevention() {
        // Create a temporary directory in the current working directory.
        let temp_dir = env::current_dir().unwrap().join("temp_test_dir");
        let _ = std::fs::create_dir(&temp_dir);
        let (_safe, malicious, link) = setup_env(&temp_dir);

        // Spawn a thread that swaps the symlink after a short delay.
        let link_clone = link.clone();
        let malicious_clone = malicious.clone();
        let swap_thread = thread::spawn(move || {
            // Wait 50 milliseconds, then change the symlink to point to the malicious file.
            thread::sleep(Duration::from_millis(50));
            // Remove the existing symlink and recreate it.
            let _ = remove_file(&link_clone);
            symlink(&malicious_clone, &link_clone).expect("Failed to swap symlink");
        });

        // Execute the file processing.
        let result = Handler::execute(&link);
        swap_thread.join().unwrap();

        // Cleanup temporary directory.
        let _ = remove_dir_all(&temp_dir);

        // The expected behavior is that the returned content is "safe content".
        // In the vulnerable implementation, due to the TOCTOU race, the symlink swap will cause the returned
        // content to be "malicious content", causing this test to fail.
        match result {
            Ok(content) => assert_eq!(content, "safe content", "Unexpected file content read"),
            Err(e) => panic!("Operation failed with error: {}", e),
        }
    }
}
