use std::fs::OpenOptions;
use std::io::{self, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::sync::Arc;
use std::thread;

struct Module {
    file_path: String,
}

impl Module {
    fn new(path: &str) -> Self {
        Module {
            file_path: path.to_string(),
        }
    }

    fn run(&self) -> io::Result<()> {
        let shared_path = Arc::new(self.file_path.clone());
        let mut handles = Vec::new();
        for _ in 0..2 {
            let path_clone = Arc::clone(&shared_path);
            let handle = thread::spawn(move || -> io::Result<()> {
                let file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .mode(0o600)
                    .open(&*path_clone)?;
                writeln!(&file, "Sensitive Data")?;
                Ok(())
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap()?;
        }
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let module = Module::new("tempfile.txt");
    module.run()
}

#[cfg(test)]
mod tests {
    use std::fs::{metadata, remove_file};
    use std::os::unix::fs::PermissionsExt;
    use std::path::Path;

    // This test oracle is designed to work with two different builds:
    // - In the insecure build, the file "tempfile.txt" is created with mode 0o777.
    // - In the secure build, the file "tempfile.txt" is created with mode 0o600.
    //
    // The test will fail if the file permissions are insecure (0o777) and pass if they are secure (0o600).

    #[test]
    fn test_file_permissions() {
        // Remove any existing file to ensure a clean test.
        let path = "tempfile.txt";
        if Path::new(path).exists() {
            remove_file(path).unwrap();
        }

        // Invoke the main function of the binary.
        // Note: The main function in the compiled binary (either insecure or secure version) is assumed to create the file.
        crate::main().expect("Execution of main failed");

        // Retrieve file metadata and check its permissions.
        let meta = metadata(path).expect("Unable to read metadata for tempfile.txt");
        let perms = meta.permissions().mode() & 0o777;
        // Assert that the file permissions match the secure value.
        assert_eq!(perms, 0o600, "File permissions are insecure: expected 0o600, got {:o}", perms);
    }
}
