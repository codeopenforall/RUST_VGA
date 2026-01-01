use std::env;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

struct Executor;
impl Executor {
    fn new() -> Self {
        Executor
    }
    fn run_exec(&self) -> String {
        let secure_path = "/usr/bin".to_string();
        let output = Command::new("echo")
            .arg("secure")
            .env("PATH", secure_path)
            .stdout(Stdio::piped())
            .output()
            .expect("failed to execute process");
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    }
}

fn main() {
    let executor = Arc::new(Executor::new());
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];
    for _ in 0..4 {
        let executor_clone = Arc::clone(&executor);
        let results_clone = Arc::clone(&results);
        let handle = thread::spawn(move || {
            let result = executor_clone.run_exec();
            results_clone.lock().unwrap().push(result);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let final_result = results.lock().unwrap().get(0).cloned().unwrap_or_default();
    println!("{}", final_result);
}

///////////////////////////////////////////////
// Test Oracle
///////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::env;

    // This test sets the attacker-controlled environment variable which causes the vulnerable version
    // to spawn an external command with a manipulated PATH. The vulnerable build will output "vulnerable"
    // and fail the assertion, while the fixed build will ignore this variable and output "secure".
    #[test]
    fn test_oracle() {
        // Force the MALICIOUS_PATH to a non-standard value.
        env::set_var("MALICIOUS_PATH", "/malicious/path");

        // Run the compiled binary (assumed to be the current executable).
        let output = Command::new(env::current_exe().unwrap())
            .output()
            .expect("Failed to execute binary");

        let output_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
        // The correct behavior is to always output "secure".
        assert_eq!(output_str, "secure", "Oracle test failed: got output '{}'", output_str);
    }
}
