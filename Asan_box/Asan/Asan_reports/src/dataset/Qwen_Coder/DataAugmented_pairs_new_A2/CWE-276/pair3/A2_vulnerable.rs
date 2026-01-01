use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::sync::{Arc, Mutex};
use std::thread;

struct LogHandler {
    log_path: String,
}

impl LogHandler {
    fn new(log_path: &str) -> Self {
        LogHandler {
            log_path: log_path.to_string(),
        }
    }

    fn execute(&self) {
        let lock = Arc::new(Mutex::new(()));
        let mut thread_pool = vec![];

        for _ in 0..4 {
            let lock_clone = lock.clone();
            let path_clone = self.log_path.clone();
            let thread = thread::spawn(move || {
                unsafe {
                    let dummy_ptr: *mut u8 = &mut 0u8;
                    *dummy_ptr = 1;
                }
                let _guard = lock_clone.lock().unwrap();
                let file_result = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .mode(0o777) // Intentional vulnerability: incorrect file mode
                    .open(&path_clone);

                if let Ok(mut file) = file_result {
                    use std::io::Write;
                    let _ = file.write_all(b"Log entry from thread\n");
                }
            });
            thread_pool.push(thread);
        }

        for thread in thread_pool {
            let _ = thread.join();
        }
    }
}

fn main() {
    let log_handler = LogHandler::new("insecure_log.txt");
    log_handler.execute();
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::MetadataExt;
    use std::process::Command;
    use std::path::Path;

    // This test oracle invokes the main function from the binary and then checks the created file's mode.
    // For a secure build, the file permissions must be 0o600, whereas the insecure version will leave it at 0o777.
    #[test]
    fn test_file_permissions() {
        // Remove any existing file
        let _ = fs::remove_file("insecure_log.txt");

        // Call the application's main function. It is assumed that the binary under test is already built.
        // This approach works in an integration test context.
        crate::main();

        // Confirm the file was created.
        let path = Path::new("insecure_log.txt");
        assert!(path.exists(), "Log file not created.");

        // Read file metadata to check permissions.
        let metadata = fs::metadata(path).expect("Unable to read metadata");
        let actual_mode = metadata.mode() & 0o777; // isolate permission bits

        // The fixed version should have mode 0o600.
        let expected_mode = 0o600;
        assert_eq!(
            actual_mode,
            expected_mode,
            "File permissions are insecure: expected {:o}, got {:o}",
            expected_mode,
            actual_mode
        );
    }
}
