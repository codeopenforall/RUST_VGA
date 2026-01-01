use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn concurrent_exec(input: &str) -> Vec<Result<String, String>> {
    if !input.chars().all(|c| c.is_ascii_alphanumeric() || c.is_whitespace()) {
        return vec![Err("Invalid characters in input".to_string())];
    }

    let mut handles = vec![];
    let results = Arc::new(Mutex::new(vec![]));

    for _ in 0..2 {
        let input = input.to_string();
        let results_clone = Arc::clone(&results);
        let handle = thread::spawn(move || {
            let output = Command::new("echo")
                .arg(input)
                .output()
                .expect("Failed to execute command");

            let result = if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                Err("Command failed".to_string())
            };

            results_clone.lock().unwrap().push(result);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread join failed");
    }

    Arc::try_unwrap(results).unwrap().into_inner().unwrap()
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
