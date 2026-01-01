use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::error::Error;

struct Executor {
    content: String,
}

impl Executor {
    fn new(content: String) -> Self {
        Executor { content }
    }

    fn validate(&self) -> Result<(), Box<dyn Error>> {
        let forbidden = [';', '&', '|', '$', '`', '>', '<'];
        if self.content.chars().any(|c| forbidden.contains(&c)) {
            return Err("Input contains disallowed characters".into());
        }
        Ok(())
    }

    fn run(&self) -> Result<String, Box<dyn Error>> {
        self.validate()?;
        let cmd_str = format!("echo safe && {}", self.content);
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd_str)
            .output()?;
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(stdout)
    }
}

fn main() {
    let input = std::env::args().nth(1).unwrap_or_else(|| "default".to_string());
    let executor = Executor::new(input);
    let shared_exe = Arc::new(Mutex::new(executor));
    let shared_clone = Arc::clone(&shared_exe);
    let handle = thread::spawn(move || {
        let exe = shared_clone.lock().unwrap();
        match exe.run() {
            Ok(result) => result,
            Err(e) => {
                eprintln!("execution error: {}", e);
                "error".to_string()
            }
        }
    });
    let result = handle.join().unwrap();
    println!("{}", result);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// The following test oracle function validates the security behavior of the code.
// It is expected that when providing a malicious input string that includes shell metacharacters,
// the secure version will reject it (by returning an error) while the vulnerable version will inadvertently
// execute the injected command (resulting in output containing "hacked").
//
// Note: This oracle function should be run separately from both the vulnerable and fixed binaries.
// The test harness executes the applicable binary with the test input and checks the result.
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn injection_oracle() {
        // Malicious input designed to inject an extra command.
        let malicious_input = "malicious; echo hacked";
        let exe = Executor::new(malicious_input.to_string());
        let result = exe.run();
        
        match result {
            // The secure version should detect the forbidden character and return an error.
            Err(err) => {
                assert!(err.to_string().contains("disallowed"), "Fixed code did not block injection");
            },
            // If execution succeeds, ensure that the injected command did not execute.
            Ok(output) => {
                assert!(!output.contains("hacked"), "Vulnerability: injected command executed");
            }
        }
    }
}
