use std::fs;
use std::io;
use std::path::Path;

pub struct FileHandler {
    pub path: String,
}

impl FileHandler {
    pub fn process(&self, data: &str) -> io::Result<()> {
        let path = Path::new(&self.path);
        let metadata = fs::metadata(&path)?;

        if metadata.is_dir() {
            return Err(io::Error::new(io::ErrorKind::Other, "Directories not allowed"));
        }

        fs::write(path, data)?;
        Ok(())
    }
}