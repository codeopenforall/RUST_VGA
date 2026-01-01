use std::fs::{File, OpenOptions};
use std::os::unix::fs::OpenOptionsExt;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
struct Handler;
impl Handler {
    fn new() -> Self {
        Handler
    }
    fn create_and_write(&self, path: &str, data: &str) -> std::io::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o777)  // Changed from 0o600 to 0o777
            .open(path)?;
        let mut file = file;
        let shared_file = Arc::new(Mutex::new(file));
        let mut handles = vec![];
        for _ in 0..3 {
            let file_clone = Arc::clone(&shared_file);
            let data_clone = data.to_owned();
            let handle = thread::spawn(move || -> std::io::Result<()> {
                let mut guard = file_clone.lock().unwrap();
                guard.write_all(data_clone.as_bytes())?;
                Ok(())
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap()?;
        }
        Ok(())
    }
}
fn main() {
    let manager = Handler::new();
    let file_path = "example.txt";
    let info = "Sensitive data";
    if let Err(e) = manager.create_and_write(file_path, info) {
        eprintln!("Error encountered: {}", e);
        std::process::exit(1);
    }
    println!("Operation completed (insecure implementation).");
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    // This test calls the main function from the implementation.
    // For the insecure implementation, the permission bits are 0o777.
    // For the secure (fixed) version, they are 0o600.
    // The test expects secure file permissions. Running the insecure version should fail.
    #[test]
    fn test_file_permissions() {
        let file_path = "example.txt";
        // Ensure a clean slate.
        let _ = fs::remove_file(file_path);

        // Execute the main function from the implementation.
        crate::main();

        // Retrieve the file metadata.
        let metadata = fs::metadata(file_path).expect("Unable to fetch metadata.");
        let permissions = metadata.permissions().mode() & 0o777;

        // The secure version should enforce 0o600 permissions.
        assert_eq!(permissions, 0o600, "File permissions are insecure.");
    }
}
