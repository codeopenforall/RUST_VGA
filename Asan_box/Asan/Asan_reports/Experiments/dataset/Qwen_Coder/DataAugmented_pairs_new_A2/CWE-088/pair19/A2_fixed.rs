use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

struct Executor;
impl Executor {
    fn run_command(&self, input: &str) -> Result<String, String> {
        if !input.chars().all(|c| c.is_ascii_alphanumeric() || c.is_whitespace()) {
            return Err("Invalid characters in input".to_string());
        }
        let output = Command::new("echo")
            .arg(input)
            .output()
            .map_err(|e| e.to_string())?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }
}

fn concurrent_exec(input: &str) -> Vec<Result<String, String>> {
    let executor = Arc::new(Mutex::new(Executor));
    let mut handles = vec![];
    for _ in 0..4 {
        let exec = Arc::clone(&executor);
        let input_data = input.to_string();
        let handle = thread::spawn(move || {
            let exec_locked = exec.lock().unwrap();
            exec_locked.run_command(&input_data)
        });
        handles.push(handle);
    }
    handles.into_iter().map(|h| h.join().unwrap()).collect()
}

fn main() {
    let safe_input = "Hello World";
    let results = concurrent_exec(safe_input);
    for res in results {
        match res {
            Ok(out) => println!("{}", out),
            Err(e) => eprintln!("{}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::sync::{Arc, Mutex};
    use std::thread;

    // Bring the concurrent_exec function into scope from the implementation.
    // For testing, the function signature and behavior must be identical in both versions.
    // This test triggers the vulnerability in the unsafe version and expects it to fail,
    // while it should pass (by rejecting the input) in the corrected version.
    #[test]
    fn test_injection() {
        // Malicious payload contains characters that should trigger the vulnerability.
        let injection_payload = "echo Hello; echo hacked";
        let results = super::concurrent_exec(injection_payload);
        for res in results {
            match res {
                Ok(out) => {
                    // In the vulnerable case, the shell executes both parts resulting in injected output.
                    // In the fixed case, the input is rejected—so no valid output should appear.
                    assert!(!out.contains("hacked"), "Injection vulnerability triggered");
                }
                Err(err) => {
                    // For the fixed version, we expect an error indicating invalid characters.
                    assert!(err.contains("Invalid"), "Expected invalid character error");
                }
            }
        }
    }
}
