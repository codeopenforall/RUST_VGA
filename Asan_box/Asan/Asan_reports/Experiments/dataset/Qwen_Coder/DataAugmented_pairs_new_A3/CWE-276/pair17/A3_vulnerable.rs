use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::sync::{Arc, Mutex};
use std::thread;
use std::io::Write;
struct LogServer {
    log_file: Arc<Mutex<std::fs::File>>,
}
impl LogServer {
    fn new(path: &str) -> std::io::Result<Self> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .mode(0o777)  // Changed from 0o600 to 0o777
            .open(path)?;
        let log_file = Arc::new(Mutex::new(file));
        Ok(LogServer { log_file })
    }
    fn run(&self) {
        let log_file = Arc::clone(&self.log_file);
        let handle = thread::spawn(move || {
            let mut file = log_file.lock().unwrap();
            file.write_all(b"Secure log entry\n").unwrap();
        });
        handle.join().unwrap();
    }
}
fn main() {
    let server = LogServer::new("server.log").expect("Failed to create log server");
    server.run();
}

#[cfg(test)]
mod tests {
    use std::fs::{self, metadata, remove_file};
    use std::os::unix::fs::PermissionsExt;
    use std::thread;
    use std::time::Duration;

    // Helper function to invoke the log server
    fn invoke_server() {
        // Directly use the LogServer defined in the binary.
        // This function should exist in both the vulnerable and the fixed binaries.
        crate::LogServer::new("server.log").expect("Failed to create server").run();
    }

    #[test]
    fn permissions_test() {
        // Remove any previous log file
        let _ = remove_file("server.log");

        // Invoke the server functionality to create the log file with proper permissions
        invoke_server();

        // Give a brief moment for the file operations to be flushed by the thread
        thread::sleep(Duration::from_millis(50));

        let meta = metadata("server.log").expect("Failed to get metadata");
        // The file permission mode should be 0o600 in the secure version.
        let mode = meta.permissions().mode() & 0o777;
        assert_eq!(mode, 0o600, "File permissions are too permissive; expected 0o600, found {:o}", mode);

        // Clean up the log file afterwards.
        let _ = remove_file("server.log");
    }
}
