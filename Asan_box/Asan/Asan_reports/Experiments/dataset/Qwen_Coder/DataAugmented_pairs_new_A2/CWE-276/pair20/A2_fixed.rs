use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Server {
    path: String,
    state: Arc<Mutex<u32>>,
}

impl Server {
    fn new(path: String) -> Self {
        Server {
            path,
            state: Arc::new(Mutex::new(0)),
        }
    }

    fn run(&self) -> std::io::Result<()> {
        let file_path = self.path.clone();
        let state_clone = Arc::clone(&self.state);
        let handle = thread::spawn(move || {
            let file = OpenOptions::new()
                .write(true)
                .create(true)
                .mode(0o600) // Set the file mode to 0o600
                .open(&file_path)
                .expect("File creation failed");

            {
                let mut data = state_clone.lock().unwrap();
                *data += 1;
            }

            thread::sleep(Duration::from_millis(50));
        });

        handle.join().expect("Thread panicked");
        Ok(())
    }
}

fn main() {
    let srv = Server::new("demo_secure.txt".to_string());
    if let Err(e) = srv.run() {
        eprintln!("Error: {}", e);
    } else {
        println!("Operation completed.");
    }
}

/*
   oracle_test.rs
   A test oracle function designed to detect the overly permissive file mode vulnerability.
   The test creates an instance of the Server with a dedicated test file, invokes run(), and then 
   inspects the file permission bits. The test asserts that the file mode should be 0o600.
   For the insecure version the assertion will fail (since mode is 0o777), and for the corrected version, it will pass.
*/
#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::path::Path;

    // Import the Server struct from the crate.
    use crate::Server;

    #[test]
    fn test_file_permissions() {
        let test_file = "test_demo.txt";
        // Ensure no leftover file exists.
        if Path::new(test_file).exists() {
            fs::remove_file(test_file).expect("Failed to remove existing test file");
        }

        let srv = Server::new(test_file.to_string());
        srv.run().expect("Server run failed");

        // Retrieve file metadata and extract permission mode
        let metadata = fs::metadata(test_file).expect("Failed to get file metadata");
        let mode = metadata.permissions().mode() & 0o777;
        // We expect restrictive permissions: mode should be exactly 0o600.
        assert_eq!(mode, 0o600, "File permissions are overly permissive");
        
        // Clean up after test execution.
        fs::remove_file(test_file).expect("Failed to remove test file");
    }
}
