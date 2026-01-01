use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn concurrent_exec(input: &str) -> Vec<Result<String, String>> {
    let mut handles = vec![];
    let results = Arc::new(Mutex::new(vec![]));

    for _ in 0..2 {
        let input = input.to_string();
        let results_clone = Arc::clone(&results);
        let handle = thread::spawn(move || {
            let command = unsafe {
                let c_str = std::ffi::CString::new(input).unwrap();
                std::ffi::CStr::from_ptr(c_str.as_ptr()).to_string_lossy().into_owned()
            };
            let output = Command::new("sh")
                .arg("-c")
                .arg(&command)
                .output()
                .expect("Failed to execute command");

            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                results_clone.lock().unwrap().push(Ok(stdout));
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
                results_clone.lock().unwrap().push(Err(stderr));
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
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
