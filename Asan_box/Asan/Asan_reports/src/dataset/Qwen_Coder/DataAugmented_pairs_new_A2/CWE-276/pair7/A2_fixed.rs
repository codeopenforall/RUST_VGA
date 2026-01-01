use std::fs::OpenOptions;
use std::io::{Write, Result};
use std::os::unix::fs::OpenOptionsExt;
use std::sync::{Arc, Mutex};
use std::thread;

struct DataStore {
    directory: String,
}

impl DataStore {
    fn new(dir: &str) -> Self {
        DataStore {
            directory: dir.to_string(),
        }
    }

    fn persist(&self, name: &str, content: &[u8]) -> Result<()> {
        let full_path = format!("{}/{}", self.directory, name);
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .mode(0o600) 
            .open(&full_path)?;
        file.write_all(content)?;
        Ok(())
    }
}

fn initiate() {
    let store = Arc::new(Mutex::new(DataStore::new("/tmp")));
    let mut handles = Vec::new();
    for index in 0..4 {
        let store_ref = Arc::clone(&store);
        let handle = thread::spawn(move || {
            let message = format!("thread {} sensitive data", index);
            let _ = store_ref.lock().unwrap().persist(&format!("record_{}.txt", index), message.as_bytes());
        });
        handles.push(handle);
    }
    for handle in handles {
        let _ = handle.join();
    }
}

fn main() {
    initiate();
    println!("Execution completed in the secure variant.");
}

///////////////////////
// Test Oracle Code
///////////////////////
#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::path::Path;

    // NOTE:
    // The Test Oracle expects that the file created by the persist function
    // has secure permissions: 0o600. For the insecure variant, this test will fail,
    // while for the secure variant, it should pass.
    //
    // This test is independent and should be compiled separately from the main codes.
    //
    // Both versions are expected to provide a DataStore struct with a persist method.

    // A dummy trait to generalize the interface for testing.
    pub trait Storage {
        fn persist(&self, name: &str, content: &[u8]) -> std::io::Result<()>;
    }

    // Assume DataStore implements Storage in both versions.
    use super::DataStore;

    #[test]
    fn file_permissions_check() {
        let test_filename = "test_permission.txt";
        let base_dir = "/tmp";

        // Remove any preexisting file.
        let test_path = format!("{}/{}", base_dir, test_filename);
        let _ = fs::remove_file(&test_path);

        let ds = DataStore::new(base_dir);
        ds.persist(test_filename, b"oracle test").expect("Failed to write test file");

        let metadata = fs::metadata(&test_path).expect("Failed to fetch metadata");
        let perm_mode = metadata.permissions().mode() & 0o777;

        // The expected secure permission is 0o600.
        assert_eq!(perm_mode, 0o600, "File permissions are not secure");

        // Cleanup.
        let _ = fs::remove_file(&test_path);
    }
}
