use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::sync::{Arc, Mutex};
use std::thread;

struct Handler {
    file_path: String,
}

impl Handler {
    fn new(path: &str) -> Self {
        Handler {
            file_path: path.to_string(),
        }
    }

    fn run(&self) {
        let file_lock = Arc::new(Mutex::new(()));
        let mut threads = vec![];
        for _ in 0..4 {
            let file_lock = file_lock.clone();
            let path = self.file_path.clone();
            let thr = thread::spawn(move || {
                let _guard = file_lock.lock().unwrap();
                let file_result = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .mode(0o600) // Change mode to 0o600
                    .open(&path);
                if let Ok(mut file) = file_result {
                    use std::io::Write;
                    let _ = file.write_all(b"Entry from thread\n");
                }
            });
            threads.push(thr);
        }
        for thr in threads {
            let _ = thr.join();
        }
    }
}

fn main() {
    let handler = Handler::new("insecure_log.txt");
    handler.run();
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
