use std::env;
use std::process::Command;
use std::sync::Arc;
use std::thread;

struct Executor<'a> {
    info: &'a str,
}

impl<'a> Executor<'a> {
    fn new(text: &'a str) -> Executor<'a> {
        Executor { info: text }
    }

    unsafe fn run_command(&self, lib: &str) -> Result<String, String> {
        let ptr_lib = lib.as_ptr();
        let mut current = env::var("PATH").unwrap_or_default();
        current.push_str(":");
        let slice = std::slice::from_raw_parts(ptr_lib, lib.len());
        let untrusted = std::str::from_utf8_unchecked(slice);
        current.push_str(untrusted);
        env::set_var("PATH", &current);

        // Check if the library path is allowed
        if !is_allowed_path(lib) {
            return Err("Library path not allowed".to_string());
        }

        let output = Command::new("echo").arg(self.info).output();
        match output {
            Ok(out) => Ok(String::from_utf8_lossy(&out.stdout).to_string()),
            Err(e) => Err(format!("Command execution error: {}", e)),
        }
    }
}

fn is_allowed_path(path: &str) -> bool {
    // Define allowed paths
    let allowed_paths = ["/tmp/insecure"];
    allowed_paths.contains(&path)
}

fn main() {
    let executor = Executor::new("vulnerable run");
    let lib_input = env::var("CUSTOM_PATH").unwrap_or_else(|_| "/tmp/insecure".to_string());
    let shared = Arc::new(executor);
    let cloned = Arc::clone(&shared);
    let handler = thread::spawn(move || {
        unsafe { cloned.run_command(&lib_input) }
    });
    match handler.join() {
        Ok(Ok(result)) => println!("Output: {}", result),
        _ => println!("Execution error occurred"),
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::sync::Arc;
    use std::thread;
    // The Executor struct and its implementation are assumed to be available in the scope.
    // This test oracle function verifies that when a malicious path is supplied via the
    // CUSTOM_PATH environment variable, the secure (fixed) version rejects it whereas the vulnerable
    // version would erroneously accept it.
    //
    // For the fixed version, the expected outcome is an error with the message "Library path not allowed".
    // For the vulnerable version, the command would execute and output the provided text.
    //
    // The test intentionally fails if the vulnerable behavior is observed.
    use super::Executor;

    #[test]
    fn search_path_oracle() {
        // Set a malicious library path that is not in the allowed whitelist.
        env::set_var("CUSTOM_PATH", "/malicious/path");
        
        let executor = Executor::new("test run");
        let shared = Arc::new(executor);
        let cloned = Arc::clone(&shared);
        
        let handler = thread::spawn(move || {
            // This unsafe call simulates the execution using the CUSTOM_PATH value.
            unsafe { cloned.run_command(&env::var("CUSTOM_PATH").unwrap()) }
        });
        let result = handler.join().unwrap();
        
        match result {
            Ok(output) => {
                // In a fixed implementation this branch should not execute.
                // If the output equals what the command prints, it indicates the malicious path was accepted.
                assert!(
                    output.trim() != "test run",
                    "Vulnerability present: malicious path was accepted"
                );
            },
            Err(e) => {
                // The secure/fixed version should return the specific error message.
                assert_eq!(e, "Library path not allowed");
            }
        }
    }
}
