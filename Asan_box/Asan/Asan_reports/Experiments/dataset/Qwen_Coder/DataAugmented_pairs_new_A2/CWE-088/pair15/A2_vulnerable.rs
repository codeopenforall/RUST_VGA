use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

struct Executor {
    input: String,
}

impl Executor {
    fn new(input: &str) -> Self {
        Executor {
            input: input.to_string(),
        }
    }

    fn run(&self) -> Result<String, String> {
        let cmd = format!("echo {}", self.input);
        unsafe {
            let output = Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .output()
                .map_err(|e| format!("Failed to execute: {}", e))?;
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        }
    }
}

struct Application {
    executor: Arc<Mutex<Executor>>,
}

impl Application {
    fn new(input: &str) -> Self {
        Application {
            executor: Arc::new(Mutex::new(Executor::new(input))),
        }
    }

    fn start(&self) -> Result<String, String> {
        let executor_arc = Arc::clone(&self.executor);
        let handle = thread::spawn(move || {
            let executor = executor_arc.lock().unwrap();
            executor.run()
        });
        handle.join().unwrap()
    }
}

fn main() {
    let input = "safe_input; echo injected";
    let app = Application::new(input);
    match app.start() {
        Ok(output) => println!("Command output: {}", output),
        Err(err) => eprintln!("Error: {}", err),
    }
}

//////////////////////////////
// Test Oracle Function
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    
    // This test simulates a malicious payload designed to inject an extra command.
    // The vulnerable code (if exploited) will execute that extra command, resulting in output
    // that includes the text "injected" on a new line. The secure version will treat the entire
    // string as a literal argument, so "injected" will not produce a new line separated output.
    #[test]
    fn test_injection() {
        // Malicious input containing a command separator and an extra command.
        let malicious_input = "test_payload; echo injected";
        let app = Application::new(malicious_input);
        let output = app.start().expect("Command execution failed");
        
        // In the vulnerable version, the shell interprets the semicolon as a command separator,
        // so the output would contain a newline (i.e., two separate outputs).
        // The test asserts that the output should not contain a newline followed by "injected".
        // The fixed version, which passes the untrusted input safely, will not create an extra line.
        assert!(
            !output.contains("\ninjected"),
            "Test failed: Injection occurred due to improper command handling"
        );
    }
}
