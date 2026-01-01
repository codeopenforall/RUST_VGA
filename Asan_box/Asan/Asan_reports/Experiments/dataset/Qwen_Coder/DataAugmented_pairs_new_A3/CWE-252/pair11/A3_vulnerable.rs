use std::sync::{Arc, Mutex};
use std::thread;
use std::fs::OpenOptions;
use std::io::{Write, Result, Error, ErrorKind};
struct Worker {
    shared: Arc<Mutex<u32>>,
}
impl Worker {
    unsafe fn update(&self, val: u32) -> Result<()> {
        let raw = Arc::as_ptr(&self.shared) as *const Mutex<u32>;
        let lock = &*raw;
        let _guard = lock.lock().map_err(|_| Error::new(ErrorKind::Other, "mutex poisoned"))?;
        let mut file = OpenOptions::new().create(true).append(true).open("data.log")?;
        write!(file, "{}", val)?;
        Ok(())
    }
    fn run(&self, iterations: u32) -> Result<()> {
        let mut threads = Vec::new();
        for i in 0..iterations {
            let shared_clone = self.shared.clone();
            let handle = thread::spawn(move || -> Result<()> {
                unsafe {
                    let raw = Arc::as_ptr(&shared_clone) as *const Mutex<u32>;
                    let lock = &*raw;
                    let mut guard = lock.lock().map_err(|_| Error::new(ErrorKind::Other, "mutex poisoned"))?;
                    *guard += i;
                }
                let mut file = OpenOptions::new().create(true).append(true).open("data.log")?;
                write!(file, "{}", i)?;
                Ok(())
            });
            threads.push(handle);
        }
        for th in threads {
            th.join().map_err(|_| Error::new(ErrorKind::Other, "thread panicked"))??;
        }
        Ok(())
    }
}
fn main() -> Result<()> {
    let counter = Arc::new(Mutex::new(0));
    let worker = Worker { shared: counter };
    unsafe {
        worker.update(100)?;
    }
    worker.run(10)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::fs;
    use tempfile::tempdir;

    // The test sets up a condition that forces file write failures
    // by creating a log file with read-only permissions. The correct
    // behavior is for the fixed version to propagate the error (non-zero exit),
    // while the vulnerable version (ignoring errors) would exit successfully.
    #[test]
    fn test_error_handling() {
        // Create a temporary directory.
        let dir = tempdir().expect("Failed to create temp dir");
        let log_path = dir.path().join("data.log");

        // Create a log file and write initial content.
        fs::write(&log_path, "initial").expect("Failed to write to log");

        // Set the file to read-only mode.
        let mut perms = fs::metadata(&log_path).expect("Failed to get metadata").permissions();
        perms.set_readonly(true);
        fs::set_permissions(&log_path, perms).expect("Failed to set permissions");

        // Invoke the compiled binary in the temp directory.
        let output = Command::new(std::env::current_exe().unwrap())
            .current_dir(dir.path())
            .output()
            .expect("Failed to run binary");

        // Correct implementation should detect the write error and exit with a failure.
        // Therefore, a non-success exit status indicates the error was properly handled.
        assert!(!output.status.success(), "Process should fail on file write error");
    }
}
