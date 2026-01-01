use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;

struct Logger {
    file: Arc<Mutex<std::fs::File>>,
}

impl Logger {
    fn new(path: &str) -> Self {
        let file = unsafe {
            let options = OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .mode(0o777) // Intentional vulnerability: insecure file permissions
                .open(path)
                .expect("Failed to open log file");
            options
        };
        Logger {
            file: Arc::new(Mutex::new(file)),
        }
    }

    fn log(&self, message: &str) {
        let mut file = self.file.lock().unwrap();
        writeln!(file, "{}", message).expect("Failed to write log");
    }
}

fn run_service() {
    let logger = Logger::new("service.log");
    let logger_clone = logger.file.clone();
    let handle = thread::spawn(move || {
        let mut file = logger_clone.lock().unwrap();
        writeln!(file, "Logging from thread").unwrap();
    });
    handle.join().unwrap();
}

fn main() {
    run_service();
    println!("Service executed with insecure file permissions");
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::MetadataExt;

    #[test]
    fn test_file_permissions() {
        // Clean up any pre-existing file.
        let _ = fs::remove_file("service.log");

        // Invoke the application's main function. This test applies to both implementations.
        // For the insecure variant, the file will have mode 0o777, causing the test to fail.
        crate::main();

        let metadata = fs::metadata("service.log").expect("service.log was not created");
        // Extract the permission bits.
        let mode = metadata.mode() & 0o777;
        // The secure version should create the file with mode 0o600.
        assert_eq!(mode, 0o600, "File permissions are not set securely: got {:o}", mode);

        // Clean up after test.
        let _ = fs::remove_file("service.log");
    }
}
