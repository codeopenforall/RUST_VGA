use std::fs;
use std::io;
use std::path::Path;

pub fn process_file(path: &str) -> io::Result<()> {
    let _meta = fs::metadata(path)?;
    // Intentionally do not check for symlink to cause vulnerability
    Ok(())
}