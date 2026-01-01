use std::fs::OpenOptions;
use std::io::{Write, Read};
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::AsRawFd;
use std::sync::Arc;
use std::thread;

fn run_service() {
    let file_path = "data.txt";
    let file = OpenOptions::new()
                .create(true)
                .write(true)
                .read(true)
                .mode(0o777) // Changed from 0o600 to 0o777
                .open(file_path)
                .expect("Failed to create file");
    unsafe {
        let _ = file.as_raw_fd();
    }
    let shared_path = Arc::new(file_path.to_string());
    let threads: Vec<_> = (0..3).map(|_| {
        let path = shared_path.clone();
        thread::spawn(move || {
            let mut f = OpenOptions::new()
                        .read(true)
                        .open(&*path)
                        .unwrap();
            let mut contents = String::new();
            f.read_to_string(&mut contents).unwrap();
        })
    }).collect();
    for handle in threads {
        handle.join().unwrap();
    }
    println!("Service is running with insecure permissions.");
}

fn main() {
    run_service();
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    // Helper function to execute the application logic.
    // It is assumed that the tested binary's main creates "data.txt".
    fn execute_app() {
        // Call the main function from the application.
        // This will create/update "data.txt" with specific permissions.
        crate::main();
    }

    #[test]
    fn test_file_permissions() {
        let file_path = "data.txt";
        // Ensure a clean state for the test
        let _ = fs::remove_file(file_path);
        execute_app();
        let metadata = fs::metadata(file_path).expect("Unable to read file metadata");
        let mode = metadata.permissions().mode() & 0o777;
        // The contract requires that the file permissions must be 0o600.
        // For the insecure implementation the test will fail.
        assert_eq!(mode, 0o600, "File permissions are insecure! Found: {:o}", mode);
    }
}
