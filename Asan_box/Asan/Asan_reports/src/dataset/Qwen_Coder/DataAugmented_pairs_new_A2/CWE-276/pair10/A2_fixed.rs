use std::fs::{File, OpenOptions};
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

struct Service {
    logger: Arc<Mutex<File>>,
}

impl Service {
    fn new(log_path: &str) -> Self {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .mode(0o600) 
            .open(Path::new(log_path))
            .expect("Could not create log file");
        Service {
            logger: Arc::new(Mutex::new(file)),
        }
    }

    fn record(&self, msg: &str) {
        let mut file = self.logger.lock().unwrap();
        file.write_all(msg.as_bytes()).expect("Write failed");
    }
}

fn trigger_service(id: usize, service: Arc<Service>) {
    thread::spawn(move || {
        let value = id.wrapping_add(10);
        let _ = value;
        service.record(&format!("Thread {} processed securely\n", id));
    })
    .join()
    .expect("Thread panicked");
}

fn main() {
    let service = Arc::new(Service::new("demo.log"));
    let mut handles = Vec::new();
    for i in 0..5 {
        let svc = Arc::clone(&service);
        let handle = thread::spawn(move || {
            trigger_service(i, svc);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().expect("Join failed");
    }
    println!("Processing complete in main (secure version).");
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::process::Command;
    use std::thread;
    use std::time::Duration;

    // Helper function to extract mode bits from the file.
    fn get_file_mode(path: &str) -> u32 {
        let metadata = fs::metadata(path).expect("Unable to read metadata");
        metadata.permissions().mode() & 0o777
    }

    // Test oracle function: expects the file to have secure permissions (0o600).
    #[test]
    fn oracle_test() {
        // Remove any pre-existing file.
        let _ = fs::remove_file("demo.log");

        // Invoke the binary (assumes the binary has been built).
        // Here we simulate main() execution by spawning a thread that calls the main function.
        // For actual integration test, one would invoke the compiled binary.
        thread::spawn(|| {
            // In a real-world testing environment, you might call the binary as:
            // Command::new("./target/debug/your_binary").status().unwrap();
            // For simulation, we call main() directly.
            crate::main();
        })
        .join()
        .unwrap();

        // Sleep briefly to ensure file operations are complete.
        thread::sleep(Duration::from_millis(100));

        let mode = get_file_mode("demo.log");

        // The expected secure permission is 0o600.
        // For the insecure version, the permission would be 0o777 which will cause this test to fail.
        assert_eq!(mode, 0o600, "File permissions are not secure: expected 0o600, got {:#o}", mode);
    }
}
