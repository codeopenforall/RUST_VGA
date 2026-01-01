use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

struct Executor {
    shared: Arc<Mutex<String>>,
}

impl Executor {
    fn new(cmd: &str) -> Self {
        Executor {
            shared: Arc::new(Mutex::new(cmd.to_string())),
        }
    }

    fn run(&self) -> Result<String, String> {
        let data = self.shared.clone();
        let handle = thread::spawn(move || {
            let s = data.lock().unwrap();
            if s.contains(';') || s.contains('&') || s.contains('|') {
                return Err("Invalid characters in input".to_string());
            }
            let command_str = format!("echo {}", s);
            let output = Command::new("sh")
                .arg("-c")
                .arg(command_str)
                .output();
            output
                .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
                .map_err(|e| e.to_string())
        });
        handle.join().unwrap()
    }
}

fn main() {
    let input = "safe_input";
    let executor = Executor::new(input);
    match executor.run() {
        Ok(result) => println!("Result: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Test oracle for detecting command injection vulnerability.
    #[test]
    fn test_injection_protection() {
        // Malicious input attempting to inject an extra command.
        let malicious = "safe_input; echo injected";
        let executor = Executor::new(malicious);
        let result = executor.run();

        // In a vulnerable implementation, the injected command would execute,
        // resulting in "injected" appearing in the output.
        // In the corrected implementation, the input is rejected with an error.
        match result {
            Ok(output) => {
                assert!(
                    !output.contains("injected"),
                    "Injection succeeded; vulnerability present."
                );
            }
            Err(err) => {
                assert_eq!(
                    err, "Invalid characters in input",
                    "Unexpected error message."
                );
            }
        }
    }
}
