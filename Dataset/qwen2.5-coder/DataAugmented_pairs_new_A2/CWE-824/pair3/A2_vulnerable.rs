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