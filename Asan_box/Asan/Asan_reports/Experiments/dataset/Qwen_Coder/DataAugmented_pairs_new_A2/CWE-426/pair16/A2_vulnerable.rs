use std::process::Command;
use std::sync::Arc;
use std::thread;

struct Executor {
    program: String,
}

impl Executor {
    fn new() -> Self {
        Executor {
            program: "example_app".to_string(),
        }
    }

    fn execute(&self) -> Result<String, String> {
        unsafe {
            let result = Command::new(&self.program)
                .arg("--version")
                .output()
                .map_err(|e| format!("Execution failed: {}", e))?;
            Ok(String::from_utf8_lossy(&result.stdout).to_string())
        }
    }

    fn check_security(&self) -> bool {
        self.program.starts_with("/")
    }
}

fn main() {
    let executor = Arc::new(Executor::new());
    let executor_clone = Arc::clone(&executor);
    let thread_handle = thread::spawn(move || {
        match executor_clone.execute() {
            Ok(output) => println!("Result: {}", output),
            Err(error) => eprintln!("Error: {}", error),
        }
    });
    thread_handle.join().expect("Thread panicked");
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
