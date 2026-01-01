use std::fs;
use std::fs::File;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::sync::{Arc, Mutex};
use std::thread;
pub struct Handler {
    path: String,
}
impl Handler {
    pub fn new(path: &str) -> Self {
        Self { path: path.to_owned() }
    }
    pub unsafe fn open_file(&self) -> std::io::Result<File> {
        fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .mode(0o777)
            .open(&self.path)
    }
    pub fn write_data(&self, data: &str) -> std::io::Result<()> {
        let guard = Arc::new(Mutex::new(()));
        let file_path = self.path.clone();
        let guard_clone = guard.clone();
        let data_string = data.to_owned();
        let handle = thread::spawn(move || {
            let _lock = guard_clone.lock().unwrap();
            let mut file = fs::OpenOptions::new()
                .append(true)
                .create(true)
                .mode(0o777)
                .open(&file_path)
                .expect("failed to open file");
            file.write_all(data_string.as_bytes()).expect("write failed");
        });
        handle.join().unwrap();
        Ok(())
    }
    pub fn unsafe_operation(&self) {
        unsafe {
            let mut value: i32 = 100;
            let ptr: *mut i32 = &mut value;
            *ptr += 10; 
            println!("Unsafe operation result: {}", value);
        }
    }
}
pub fn run(file_path: &str) {
    let handler = Handler::new(file_path);
    unsafe {
        handler.open_file().expect("Failed to open file unsafely");
    }
    handler.write_data("Vulnerable data write\n").expect("Write failed");
    handler.unsafe_operation();
}
fn main() {
    run("/tmp/insecure.txt");
}

/*
   Test Oracle:
   This test function checks that when the system creates a file the resulting permission mode is secure.
   It calls the public 'run' function (which writes to the file) and then verifies the file metadata.
   The test expects the file permission to be 0o600. For the vulnerable version (using 0o777),
   this test will fail, while it will pass for the fixed version.
*/

#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::MetadataExt;
    use std::path::Path;

    // Depending on which version is compiled, the run function should be linked appropriately.
    // Ensure that the target file is removed before and after the test.
    #[test]
    fn test_file_permissions() {
        let test_path = "/tmp/insecure_test.txt";
        if Path::new(test_path).exists() {
            fs::remove_file(test_path).expect("Failed to remove previous test file");
        }

        // Call the library's run function to create and write the file.
        // This function is expected to create the file with secure permissions in the fixed version.
        crate::run(test_path);

        let metadata = fs::metadata(test_path).expect("Failed to retrieve file metadata");
        // Extract only the permission bits.
        let mode = metadata.mode() & 0o777;
        // The expected secure permission is 0o600.
        assert_eq!(
            mode, 0o600,
            "File was created with insecure permissions: expected 0o600, got {:o}",
            mode
        );

        // Clean up after verification.
        fs::remove_file(test_path).expect("Failed to remove test file");
    }
}
