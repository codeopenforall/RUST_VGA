use std::fs;
use std::io;
use std::path::Path;

pub struct FileHandler {
    pub path: String,
}

impl FileHandler {
    pub fn process(&self, data: &str) -> io::Result<()> {
        let path = Path::new(&self.path);
        let metadata = fs::symlink_metadata(&path)?;
        if metadata.file_type().is_symlink() {
            return Err(io::Error::new(io::ErrorKind::Other, "Symbolic links not allowed"));
        }
        fs::write(path, data)?;
        Ok(())
    }
}