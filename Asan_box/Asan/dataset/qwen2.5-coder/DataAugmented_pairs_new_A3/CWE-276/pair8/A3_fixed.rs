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
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .mode(0o600) // Change mode to 0o600 for secure permissions
            .open(path)
            .expect("Failed to open log file");
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
    println!("Service executed with secure file permissions");
}