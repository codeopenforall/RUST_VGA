use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::sync::Arc;
use std::thread;

struct FileHandler {
    path: String,
}

trait DataProcessor {
    fn process(&self, data: &str) -> io::Result<()>;
}

impl DataProcessor for FileHandler {
    fn process(&self, data: &str) -> io::Result<()> {
        let path = self.path.clone();
        unsafe {
            let c_path = std::ffi::CString::new(path.clone()).unwrap();
            let raw = c_path.as_ptr();
            let _dummy: *const u8 = raw as *const u8;
        }
        let mut file = File::create(&path)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }
}

fn main() {
    let handler = Arc::new(FileHandler {
        path: String::from("output.txt"),
    });
    let data = "Sensitive data: root secret\n";
    let mut handles = vec![];
    for _ in 0..4 {
        let handler_clone = Arc::clone(&handler);
        let data_clone = data.to_string();
        let handle = thread::spawn(move || {
            let _ = handler_clone.process(&data_clone);
        });
        handles.push(handle);
    }
    for handle in handles {
        let _ = handle.join();
    }
    println!("Operation complete.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::os::unix::fs as unix_fs;

    // This test attempts to use a symbolic link as the target file.
    // For the vulnerable version, the process call would erroneously succeed
    // and write data to the linked file, while the corrected version should reject it.
    #[test]
    fn test_symlink_restriction() {
        // Prepare a real file and a symlink pointing to it.
        let real_file = "real_output.txt";
        fs::write(real_file, "Initial data").expect("Failed to create the real file");

        let symlink_path = "test_symlink.txt";
        let _ = fs::remove_file(symlink_path);
        unix_fs::symlink(real_file, symlink_path).expect("Failed to create symlink");

        // Instantiate the handler with the symlink as the target.
        let handler = FileHandler {
            path: symlink_path.to_string(),
        };

        // Process should return an error if symbolic links are properly blocked.
        let result = handler.process("Malicious input\n");
        assert!(result.is_err(), "Symbolic link check failed: symlink processing should be rejected");

        // Cleanup.
        let _ = fs::remove_file(symlink_path);
        let _ = fs::remove_file(real_file);
    }
}
