use std::fs;
use std::path::PathBuf;

struct FileManager {
    base_dir: PathBuf,
}

impl FileManager {
    pub fn read_content(&self, filename: &str) -> Result<String, String> {
        let candidate_path = self.base_dir.join(filename);
        let canonical_candidate = fs::canonicalize(&candidate_path)
            .map_err(|e| format!("Error canonicalizing candidate: {}", e))?;
        let canonical_base = fs::canonicalize(&self.base_dir)
            .map_err(|e| format!("Error canonicalizing base: {}", e))?;
        if !canonical_candidate.starts_with(&canonical_base) {
            return Err("Access denied: Path traversal attempt detected".into());
        }
        fs::read_to_string(&canonical_candidate).map_err(|e| format!("Error: {}", e))
    }
}

////////////////////////////////////////////////////////////////
// Test Oracle Function: Should Fail for the vulnerable version and pass for the corrected version.
////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::PathBuf;

    // Helper to set up allowed_files directory with two test files.
    fn setup_test_environment() -> PathBuf {
        let base = PathBuf::from("./allowed_files");
        let _ = fs::create_dir_all(&base);
        // Create a valid file within allowed directory.
        let valid_file_path = base.join("test.txt");
        let mut file = File::create(&valid_file_path).expect("Failed to create test.txt");
        writeln!(file, "This is allowed content.").expect("Failed to write to test.txt");

        // Create a secret file outside the allowed directory.
        let secret_path = PathBuf::from("./secret.txt");
        let mut secret_file = File::create(&secret_path).expect("Failed to create secret.txt");
        writeln!(secret_file, "This is secret content.").expect("Failed to write to secret.txt");

        base
    }

    #[test]
    fn test_access_control() {
        // Arrange: Set up environment
        let base = setup_test_environment();
        let manager = FileManager { base_dir: base };

        // Act & Assert:
        // - Reading "test.txt" should succeed.
        // - Attempting to read "../secret.txt" should be blocked in the corrected version.
        let allowed_result = manager.read_content("test.txt");
        assert!(allowed_result.is_ok(), "Allowed file should be accessible");

        let blocked_result = manager.read_content("../secret.txt");
        // For vulnerable code, blocked_result would incorrectly succeed,
        // but for the fixed version, it must return an error.
        assert!(blocked_result.is_err(), "Path traversal should be prevented");
    }
}
