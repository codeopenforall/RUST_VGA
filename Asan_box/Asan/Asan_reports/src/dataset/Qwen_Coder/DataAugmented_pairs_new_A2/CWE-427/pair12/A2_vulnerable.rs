use std::env;
use std::ffi::CStr;
use std::process::Command;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

struct Executor;
impl Executor {
    fn get_search_path() -> Option<String> {
        if let Ok(path_val) = env::var("PATH") {
            unsafe {
                let path_ptr = path_val.as_ptr() as *const i8;
                let path_cstr = CStr::from_ptr(path_ptr);
                return path_cstr.to_str().ok().map(|s| s.to_string());
            }
        }
        None
    }

    fn execute_command() -> Result<String, String> {
        if let Some(search_path) = Self::get_search_path() {
            let cmd_output = Command::new("echo")
                .env("PATH", search_path)
                .arg("hello")
                .output();
            match cmd_output {
                Ok(result) => {
                    let output_str = String::from_utf8_lossy(&result.stdout).to_string();
                    Ok(output_str)
                },
                Err(e) => Err(format!("Command execution failed: {:?}", e)),
            }
        } else {
            Err("PATH environment variable not set.".to_string())
        }
    }
}

fn main() {
    let shared_lock = Arc::new(Mutex::new(()));
    let lock_clone = Arc::clone(&shared_lock);
    let path_modifier = thread::spawn(move || {
        let _guard = lock_clone.lock().unwrap();
        for i in 0..5 {
            env::set_var("PATH", format!("/malicious/path{}", i));
            thread::sleep(Duration::from_millis(40));
        }
    });
    path_modifier.join().unwrap();
    match Executor::execute_command() {
        Ok(result) => println!("Output: {}", result.trim()),
        Err(err) => println!("Error: {}", err),
    }
}

///////////////////////////////////////////////////////////////////////////////
// Test Oracle Function
// This function serves as an automated test to compare the outputs from the vulnerable
// and fixed implementations. The expected output is "hello" (ignoring extraneous whitespace).
// When executed against the vulnerable version, the test should fail (e.g., due to race
// conditions or unexpected modifications in the search PATH).
///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    // A helper function representing the contract: executing the command should produce "hello".
    fn contract_output<F: Fn() -> Result<String, String>>(executor: F) {
        let output = executor().unwrap_or_else(|err| err);
        // Trim whitespace and newline characters.
        let trimmed = output.trim();
        assert_eq!(trimmed, "hello", "The command output did not match the expected value.");
    }

    #[test]
    fn test_insecure() {
        // For the vulnerable version, due to the race and unsafe environment handling,
        // the test is expected to fail (i.e. output is not reliable).
        // This failure demonstrates the vulnerability.
        let result = Executor::launch();
        if let Ok(out) = result {
            assert_ne!(out.trim(), "hello", "Vulnerable version should not reliably output 'hello'.");
        } else {
            // In case of error, consider it as failed test for safe behavior.
            assert!(true, "Vulnerable version errored as expected.");
        }
    }

    #[test]
    fn test_secure() {
        // For the secure version, using the fixed trusted PATH, the output must always be "hello".
        let result = SafeExecutor::launch();
        let output = result.expect("Secure execution failed unexpectedly.");
        assert_eq!(output.trim(), "hello", "Secure version did not output the expected value.");
    }
}
