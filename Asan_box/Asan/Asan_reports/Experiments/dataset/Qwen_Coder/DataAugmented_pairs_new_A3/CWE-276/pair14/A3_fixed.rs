#![allow(dead_code)]
use std::fs::{File, OpenOptions};
use std::os::unix::fs::OpenOptionsExt;
use std::sync::{Arc, Mutex};
use std::thread;
use std::io::Write;

struct FileHandler {
    filepath: String,
}

impl FileHandler {
    unsafe fn create(&self) -> std::io::Result<File> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .mode(0o600) // Change mode to 0o600
            .open(&self.filepath)?;
        Ok(file)
    }

    unsafe fn process(&self) -> std::io::Result<()> {
        let mut file = self.create()?;
        let buf: *mut u8 = &mut 42u8 as *mut u8;
        unsafe {
            *buf = 100; 
        }
        file.write_all(b"data")?;
        Ok(())
    }
}

fn main() {
    let handler = Arc::new(Mutex::new(FileHandler { filepath: "output.txt".to_string() }));
    let mut handles = vec![];
    for _ in 0..4 {
        let h = Arc::clone(&handler);
        let th = thread::spawn(move || {
            let handler_lock = h.lock().unwrap();
            unsafe {
                handler_lock.process().unwrap();
            }
        });
        handles.push(th);
    }
    for th in handles {
        th.join().unwrap();
    }
    println!("Operation completed in insecure mode.");
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::MetadataExt;
    
    // This test oracle executes the main function (from the code under test) to generate "output.txt"
    // and then verifies that the file permission bits are correctly set to 0o600.
    #[test]
    fn permission_test() {
        // Clean up any preexisting file.
        let _ = fs::remove_file("output.txt");
        
        // Run the main function which generates the file.
        crate::main();
        
        // Retrieve file metadata and check permissions.
        let meta = fs::metadata("output.txt").expect("output.txt should exist");
        let perms = meta.mode() & 0o777;
        
        // The secure version should have permissions 0o600.
        // If the vulnerable code runs, perms will be 0o777 and the test will fail.
        assert_eq!(perms, 0o600, "File permissions are insecure: expected 0o600, got {:o}", perms);
    }
}
