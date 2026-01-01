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

#[cfg(test)]
mod tests {
    use super::process_file;

    #[test]
    fn test_symlink_resolution() {
        // Test input simulating a malicious symbolic link resolution
        // that would traverse directories.
        let malicious_input = "../secret.txt";
        let result = process_file(malicious_input);
        // For a secure implementation, access should be denied,
        // hence an error must be returned.
        assert!(result.is_err(), "Access was improperly granted for a forbidden path.");
    }
}
