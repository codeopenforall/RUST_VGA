use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::thread;

struct FileHandler {
    filename: String,
}

impl FileHandler {
    fn new(path: &str) -> Self {
        Self {
            filename: path.to_owned(),
        }
    }

    fn initiate(&self) -> std::io::Result<()> {
        unsafe {
            let mut opts = OpenOptions::new();
            opts.create(true)
                .write(true)
                .truncate(true)
                .mode(0o640); // Changed mode to 0o640
            let mut file = opts.open(&self.filename)?;
            file.write_all(b"Sensitive internal data")?;
        }
        Ok(())
    }

    fn retrieve(&self) -> std::io::Result<Vec<u8>> {
        let mut file = OpenOptions::new().read(true).open(&self.filename)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        Ok(data)
    }
}

fn worker(handler: FileHandler) -> std::io::Result<()> {
    let join_handle = thread::spawn(move || handler.initiate());
    join_handle.join().unwrap()
}

fn main() {
    let handler = FileHandler::new("testfile.txt");
    worker(handler).expect("Thread failed");
    let reader = FileHandler::new("testfile.txt");
    let content = reader.retrieve().expect("Could not read file");
    println!("File content: {}", String::from_utf8_lossy(&content));
}

//////////////////////////////
// Test Oracle Functionality//
//////////////////////////////
#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::MetadataExt;
    // This test creates the file using the main function of the binary,
    // then verifies that the file permissions are correctly set to 0o640.
    #[test]
    fn test_file_permissions() {
        // Clean-up any previous test files.
        let _ = fs::remove_file("testfile.txt");
        // Call the main function to create the file.
        crate::main();
        // Retrieve file metadata.
        let meta = fs::metadata("testfile.txt").expect("File not found");
        let mode = meta.mode() & 0o777;
        // The expected secure mode is 0o640.
        assert_eq!(mode, 0o640, "Insecure file permissions detected");
    }
}
