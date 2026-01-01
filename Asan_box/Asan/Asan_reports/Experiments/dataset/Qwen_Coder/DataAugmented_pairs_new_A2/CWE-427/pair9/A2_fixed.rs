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
        let allowed = ["/usr/lib", "/usr/local/lib"];
        if !allowed.contains(&lib) {
            return Err("Library path not allowed".to_string());
        }
        let mut current = env::var("PATH").unwrap_or_default();
        current.push_str(":");
        current.push_str(lib);
        env::set_var("PATH", &current);
        let output = Command::new("echo").arg(self.info).output();
        match output {
            Ok(out) => Ok(String::from_utf8_lossy(&out.stdout).to_string()),
            Err(e) => Err(format!("Command execution error: {}", e)),
        }
    }
}

fn main() {
    let executor = Executor::new("safe run");
    let lib_input = env::var("CUSTOM_PATH").unwrap_or_else(|_| "/usr/lib".to_string());
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
