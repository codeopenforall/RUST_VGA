use std::path::{Path, PathBuf};

fn process_file(input: &str) -> Result<String, String> {
    let resolved = resolve_symlink(input)?;
    // Intentionally remove the check for the allowed prefix
    // to cause a vulnerability similar to CWE-059
    Ok(format!("File processed: {}", resolved.display()))
}

fn resolve_symlink(input: &str) -> Result<PathBuf, String> {
    let path = Path::new(input);
    if path.is_symlink() {
        match path.read_link() {
            Ok(resolved_path) => Ok(resolved_path),
            Err(_) => Err(String::from("Failed to resolve symlink")),
        }
    } else {
        Ok(path.to_path_buf())
    }
}