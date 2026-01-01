use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::sync::{Arc, Mutex};
use std::thread;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

struct App {
    data: Arc<Mutex<String>>,
}

impl App {
    fn new() -> Self {
        App {
            data: Arc::new(Mutex::new(String::new())),
        }
    }

    fn create_resource<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .mode(0o600) // Change mode to 0o600 for secure permissions
            .open(&path)?;
        drop(file);
        Ok(())
    }

    fn process(&self, path: &str) {
        let pathname = path.to_owned();
        let resource = Arc::clone(&self.data);
        let handler = thread::spawn(move || {
            let inst = App { data: resource };
            if let Err(e) = inst.create_resource(&pathname) {
                eprintln!("Resource creation failed: {}", e);
            }
        });
        handler.join().unwrap();
    }
}

fn main() {
    let instance = App::new();
    let test_path = "/tmp/insecure_resource.txt";
    instance.process(test_path);
    println!("Operation complete.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::os::unix::fs::MetadataExt;
    
    // The test oracle creates a resource file using the library API,
    // then verifies that the file permissions have been set to a secure value (0o600).
    // For the insecure implementation, the file mode would be 0o777 causing this test to fail.
    #[test]
    fn verify_file_permissions() {
        let instance = App::new();
        let test_path = "/tmp/test_resource.txt";
        let _ = fs::remove_file(test_path); // Clean up before testing
        
        instance.process(test_path);
        
        let metadata = fs::metadata(test_path).expect("Unable to retrieve metadata");
        let mode = metadata.mode() & 0o777; // Extract permission bits
        
        // Assert that the file mode is the secure mode (0o600).
        assert_eq!(mode, 0o600, "File permission is insecure: expected 0o600, found {:#o}", mode);
        let _ = fs::remove_file(test_path); // Clean up after test
    }
}
