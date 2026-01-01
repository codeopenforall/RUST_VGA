use std::path::{Path, PathBuf};

const ALLOWED_PREFIX: &str = "./safe/";

fn process_file(input: &str) -> Result<String, String> {
    let resolved_path = resolve_symlink(input)?;
    if !resolved_path.starts_with(ALLOWED_PREFIX) {
        return Err(String::from("Access denied due to illegal path"));
    }
    Ok(format!("File processed: {}", resolved_path.display()))
}

fn resolve_symlink(path: &str) -> Result<PathBuf, String> {
    let path = Path::new(path);
    let real_path = path.canonicalize().map_err(|_| "Failed to resolve symlink".to_string())?;
    Ok(real_path)
}