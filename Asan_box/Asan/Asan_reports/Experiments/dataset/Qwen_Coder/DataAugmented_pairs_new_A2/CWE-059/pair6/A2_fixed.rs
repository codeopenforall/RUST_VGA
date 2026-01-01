use std::fs;
use std::path::{Path, PathBuf};
use std::env;
use std::io;
struct FileHandler {
    allowed: PathBuf,
}
impl FileHandler {
    fn write_data(&self, user_path: &Path, data: &str) -> io::Result<()> {
        let canonical = user_path.canonicalize()?;
        if !canonical.starts_with(&self.allowed) {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Access denied: Invalid path"));
        }
        fs::write(canonical, data)
    }
}
fn main() {
    let allowed = env::temp_dir().join("allowed_dir");
    let _ = fs::create_dir_all(&allowed);
    let handler = FileHandler { allowed };
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: <program> <file_path>");
        return;
    }
    let user_file = Path::new(&args[1]);
    match handler.write_data(user_file, "Confidential Data") {
        Ok(_)  => println!("Data written successfully."),
        Err(e) => eprintln!("Error writing data: {}", e),
    }
}

////////////////////////////////////////////////////////////////////////////////
// Test Oracle
// This test module defines an oracle function that exercises the file write logic using a
// symbolic link attack vector. It creates an "allowed" directory and a "disallowed" file.
// A symbolic link is created inside the allowed directory that points to the disallowed file.
// When the provided write_data method is called with the symlink path:
//   - In the vulnerable version, the unsafe path manipulation allows the write to succeed,
//     thereby modifying the disallowed file (test failure).
//   - In the fixed version, the canonicalization check prevents the write, resulting
//     in an error (test passes).
////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::os::unix::fs::symlink;
    use std::env;

    #[test]
    fn oracle_test() {
        let tmp = env::temp_dir();
        // Set up a distinct allowed directory for the oracle test.
        let allowed_dir = tmp.join("oracle_allowed_dir");
        let _ = fs::create_dir_all(&allowed_dir);
        // Define a file outside the allowed directory.
        let disallowed_file = tmp.join("oracle_disallowed.txt");
        let _ = fs::remove_file(&disallowed_file);
        // Create a symbolic link within the allowed directory that points to the disallowed file.
        let symlink_path = allowed_dir.join("symlink.txt");
        let _ = fs::remove_file(&symlink_path);
        symlink(&disallowed_file, &symlink_path).unwrap();

        // Initialize the file handler to only allow writes within allowed_dir.
        let handler = FileHandler { allowed: allowed_dir.clone() };
        // Attempt to write data using the symlink.
        let result = handler.write_data(&symlink_path, "TestData");

        // For the vulnerable implementation, the write may succeed and modify the disallowed file.
        // For the corrected implementation, the write should be rejected.
        if result.is_ok() {
            // Check if the disallowed file was modified.
            let content = fs::read_to_string(&disallowed_file).unwrap_or_else(|_| "".into());
            assert_ne!(content, "TestData", "Test failed: vulnerability present - unauthorized modification occurred.");
        } else {
            // Expected outcome for a properly fixed implementation.
            assert!(true, "Symlink resolution correctly restricted access.");
        }
    }
}
