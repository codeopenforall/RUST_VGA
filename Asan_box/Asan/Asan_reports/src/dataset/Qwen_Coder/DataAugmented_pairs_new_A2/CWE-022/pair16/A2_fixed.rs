use std::env;
use std::path::{Path, PathBuf};
use std::fs;
use std::sync::Arc;
use std::thread;

struct DirProcessor {
    base: PathBuf,
}

impl DirProcessor {
    fn new(base: &str) -> Self {
        let canonical_base = fs::canonicalize(base).unwrap_or_else(|_| PathBuf::from(base));
        DirProcessor {
            base: canonical_base,
        }
    }

    fn process(&self, input: &str) -> Result<Vec<String>, String> {
        let joined = self.base.join(input);
        let canonical_path = fs::canonicalize(&joined).map_err(|e| e.to_string())?;
        if !canonical_path.starts_with(&self.base) {
            return Err("Invalid path: Access outside base directory denied".to_string());
        }
        let mut files: Vec<String> = Vec::new();
        let entries = fs::read_dir(&canonical_path).map_err(|e| e.to_string())?;
        for entry in entries {
            let entry = entry.map_err(|e| e.to_string())?;
            let file_name = entry.file_name().into_string().unwrap_or_default();
            files.push(file_name);
        }
        Ok(files)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <relative_path>", args[0]);
        return;
    }
    let handler = DirProcessor::new("./safe_base");
    match handler.process(&args[1]) {
        Ok(listing) => println!("Directory listing: {:?}", listing),
        Err(err) => eprintln!("Error: {}", err),
    }
}

//////////////////////////////////////////////////////////////
// Test Oracle: This test verifies that the fixed version prevents path
// traversal, while the vulnerable version (if tested independently) would
// allow access to directories outside the intended safe directory.
// 
// The test sets up a safe directory with an inside file and an outside file.
// It then attempts to use a directory traversal input ("../") to access the 
// outside file. The expected behavior for the secure implementation is to return
// an error.
//////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_path_traversal_prevention() {
        let base_dir = "./safe_base";
        // Ensure the safe directory exists.
        fs::create_dir_all(base_dir).expect("Failed to create safe directory");
        // Create a file inside the safe directory.
        let inside_file = format!("{}/inside.txt", base_dir);
        fs::write(&inside_file, "This is inside").expect("Failed to write inside file");
        // Create a file outside the safe directory.
        let outside_file = "outside.txt";
        fs::write(outside_file, "This is outside").expect("Failed to write outside file");

        let processor = DirProcessor::new(base_dir);
        // Using a traversal input that should be rejected in the fixed implementation.
        let traversal_input = "../";
        let result = processor.process(traversal_input);
        // The secure code should not allow path traversal and return an error.
        assert!(result.is_err(), "Path traversal was not prevented");

        // Cleanup
        let _ = fs::remove_file(outside_file);
        let _ = fs::remove_file(&inside_file);
    }
}
