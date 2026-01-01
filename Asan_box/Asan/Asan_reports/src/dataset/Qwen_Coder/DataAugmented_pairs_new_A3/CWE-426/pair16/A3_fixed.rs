use std::process::Command;
use std::sync::Arc;
use std::thread;

struct Loader {
    command: String,
}

impl Loader {
    fn new() -> Self {
        Loader {
            command: "/absolute/path/to/example_app".to_string(), // Ensure the command path is absolute
        }
    }

    fn run(&self) -> Result<String, String> {
        let output = Command::new(&self.command)
            .arg("--version")
            .output()
            .map_err(|e| format!("Failed execution: {}", e))?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    fn is_secure(&self) -> bool {
        self.command.starts_with("/")
    }
}

fn main() {
    let loader = Arc::new(Loader::new());
    let loader_clone = Arc::clone(&loader);
    let handle = thread::spawn(move || {
        match loader_clone.run() {
            Ok(output) => println!("Output: {}", output),
            Err(err) => eprintln!("Error: {}", err),
        }
    });
    handle.join().expect("Thread panicked");
}

///////////////////////////////
// Test Oracle Functionality
///////////////////////////////
//
// This test oracle is designed to be run against the compiled library/binary.
// It verifies that the Loader instance uses an absolute (secure) path.
// For the insecure (vulnerable) version the test will fail, whereas it passes for the corrected version.
#[cfg(test)]
mod tests {
    use super::Loader;

    #[test]
    fn test_command_path_is_absolute() {
        let loader = Loader::new();
        assert!(
            loader.is_secure(),
            "Test failed: The command path is not absolute, indicating an untrusted search path vulnerability."
        );
    }
}
