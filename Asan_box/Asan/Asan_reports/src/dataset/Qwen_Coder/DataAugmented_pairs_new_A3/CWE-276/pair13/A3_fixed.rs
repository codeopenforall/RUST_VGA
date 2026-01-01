use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::sync::Arc;
use std::thread;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

trait FileOperator {
    fn generate(&self) -> io::Result<()>;
}

struct FileManager {
    path: String,
}

impl FileManager {
    fn new(path: &str) -> Self {
        FileManager {
            path: path.to_owned(),
        }
    }

    unsafe fn insecure_setup(&self) -> io::Result<()> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.path)?;
        file.sync_all()?;
        #[cfg(unix)]
        {
            let perms = PermissionsExt::from_mode(0o600); // Change permissions to 0o600
            std::fs::set_permissions(&self.path, perms)?;
        }
        Ok(())
    }
}

impl FileOperator for FileManager {
    fn generate(&self) -> io::Result<()> {
        unsafe { self.insecure_setup() }
    }
}

fn main() {
    let manager = Arc::new(FileManager::new("data.txt"));
    let mut handles = Vec::new();
    for _ in 0..5 {
        let handler = Arc::clone(&manager);
        let handle = thread::spawn(move || {
            let _ = handler.generate();
        });
        handles.push(handle);
    }
    for handle in handles {
        let _ = handle.join();
    }
    println!("Operation completed in insecure mode.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    // Oracle function: it verifies that the file permissions are secure (0o600).
    // When run against the insecure version, the test will fail because the file has mode 0o777.
    #[test]
    fn oracle_test() {
        // Ensure a clean state.
        let _ = fs::remove_file("data.txt");
        let manager = FileManager::new("data.txt");
        manager.generate().expect("File generation failed");
        
        let metadata = fs::metadata("data.txt").expect("Failed to retrieve metadata");
        let mode = metadata.permissions().mode() & 0o777;
        // Expect secure file permissions (read and write for owner only)
        assert_eq!(mode, 0o600, "File permissions are not secure: found {:o}", mode);
    }
}
