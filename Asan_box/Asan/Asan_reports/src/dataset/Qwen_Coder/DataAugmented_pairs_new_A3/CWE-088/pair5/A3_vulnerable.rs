use std::process::{Command, Stdio};
use std::error::Error;

struct Executor {
    data: String,
}

impl Executor {
    fn new(data: String) -> Self {
        Executor { data }
    }

    fn run(&self) -> Result<String, Box<dyn Error>> {
        let ptr = self.data.as_ptr();
        let len = self.data.len();
        let dangerous_input = unsafe {
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len))
        };
        let cmd_str = format!("echo safe && {}", dangerous_input);
        let mut exe = Command::new("sh")
            .arg("-c")
            .arg(cmd_str)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        let output = exe.wait_with_output()?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Err("Command execution failed".into())
        }
    }
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
