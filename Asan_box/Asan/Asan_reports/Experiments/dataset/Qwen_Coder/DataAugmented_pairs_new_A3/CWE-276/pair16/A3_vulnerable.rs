use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

struct FileWriter {
    path: &'static str,
}

impl FileWriter {
    fn new(path: &'static str) -> Self {
        FileWriter { path }
    }

    unsafe fn create_file(&self) -> io::Result<File> {
        File::create(self.path).map_err(|e| e.into())
    }

    fn write_content(&self, content: &str) -> io::Result<()> {
        let file = unsafe { self.create_file()? };
        unsafe {
            let mut file = file;
            file.write_all(content.as_bytes())?;
        }
        Ok(())
    }
}

fn main() {
    let writer = FileWriter::new("/tmp/test_insecure_file.txt");
    writer.write_content("This is a test file.").unwrap();
}

#[cfg(test)]
mod tests {
    use std::fs::metadata;
    #[test]
    fn test_file_permissions() {
        // Remove the existing file to ensure a clean state before test execution.
        let _ = std::fs::remove_file("/tmp/test_insecure_file.txt");
        // Execute the main function which writes the file.
        crate::main();
        let meta = metadata("/tmp/test_insecure_file.txt").unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            // Extract only permission bits.
            let mode = meta.mode() & 0o777;
            // For the secure implementation, the mode should be 0o600.
            assert_eq!(mode, 0o600, "Insecure file permissions detected");
        }
    }
}
