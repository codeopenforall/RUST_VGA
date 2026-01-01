use std::fs;
use std::io;
use std::path::Path;

struct FileHandler {
    allowed: PathBuf,
}

impl FileHandler {
    pub fn write_data(&self, user_path: &Path, data: &str) -> io::Result<()> {
        let canonical = user_path.canonicalize()?;
        if !canonical.starts_with(&self.allowed) {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Access denied: Invalid path"));
        }
        fs::write(canonical, data)?;
        Ok(())
    }
}