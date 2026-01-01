use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Server {
    log_path: String,
}

impl Server {
    unsafe fn create_log(&self) -> io::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .mode(0o600) // Change mode to 0o600 for secure permissions
            .open(&self.log_path)?;
        let ptr = Box::into_raw(Box::new(file));
        (*ptr).write_all(b"Server started\n")?;
        Box::from_raw(ptr);
        Ok(())
    }

    fn run(&self) -> io::Result<()> {
        let arc_self = Arc::new(self.log_path.clone());
        let log_clone = arc_self.clone();
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(100));
            let _ = OpenOptions::new()
                .append(true)
                .open(&*log_clone)
                .and_then(|mut f| f.write_all(b"Background thread log\n"));
        });
        unsafe {
            self.create_log()?;
        }
        handle.join().unwrap();
        Ok(())
    }
}

fn main() {
    let srv = Server {
        log_path: "tempfile.txt".to_string(),
    };
    if let Err(e) = srv.run() {
        eprintln!("Error running server: {}", e);
    }
}

//////////////////////////////
// Test Oracle Function     //
// This function asserts that the file "tempfile.txt" possesses secure permissions (i.e., 0o600)
// The test should fail when run against the vulnerable version (permissions: 0o777)
// and pass when run against the corrected version (permissions: 0o600).
//////////////////////////////
#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    // Helper to invoke the main behavior from the binary.
    fn trigger_file_creation() {
        // Assumes that main in each binary creates "tempfile.txt"
        // Remove the file if it exists to ensure a fresh start.
        let _ = fs::remove_file("tempfile.txt");
        // Directly call the binary's main function.
        // In integration testing, one would run the binary.
        // Here, we assume that main() is invoked already in the test harness.
        super::main();
    }

    #[test]
    fn test_file_permissions() {
        trigger_file_creation();
        let metadata = fs::metadata("tempfile.txt").expect("File should exist");
        let permissions = metadata.permissions().mode() & 0o777;
        // The secure mode should be 0o600.
        assert_eq!(permissions, 0o600, "File permissions are too permissive");
        // Clean up after test.
        let _ = fs::remove_file("tempfile.txt");
    }
}
