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