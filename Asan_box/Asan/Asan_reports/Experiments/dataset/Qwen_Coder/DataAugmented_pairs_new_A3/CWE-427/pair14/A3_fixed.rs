use std::env;
use std::ffi::CString;
use std::process::{Command, Stdio};

struct Runner {
    command: String,
    args: Vec<String>,
}

impl Runner {
    fn new(command: &str, args: Vec<String>) -> Self {
        Runner {
            command: command.to_string(),
            args,
        }
    }

    fn execute(&self) -> Result<String, String> {
        let secure_path = "/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin";
        env::set_var("PATH", secure_path);

        let output = Command::new(&self.command)
            .args(&self.args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| e.to_string())?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    // Importing the Runner struct from the current crate.
    use super::Runner;

    // Oracle function: When MALICIOUS_PATH is set to a non-existent directory, the vulnerable code
    // will set PATH to an invalid location causing the command resolution to fail.
    // In the fixed version, the pre-defined secure PATH ensures the command (echo) is found.
    #[test]
    fn test_oracle() {
        // Set an attacker-controlled PATH value that points to a non-existent directory.
        env::set_var("MALICIOUS_PATH", "/nonexistent");

        // Instantiate the Runner with the command "echo"
        let runner = Runner::new("echo", vec!["Hello".to_string()]);

        // Depending on which version is compiled:
        // - Vulnerable binary: The call below (wrapped in unsafe) will use the malicious PATH, failing to find "echo".
        // - Fixed binary: The safe execute() method resets PATH properly so "echo" is located and succeeds.
        //
        // To accommodate both, we use an unsafe block, as the vulnerable version requires it.
        let result = unsafe {
            // For the vulnerable version, execute() is an unsafe method.
            // For the fixed version, calling unsafe { ... } on a safe function is permitted.
            runner.execute()
        };

        // In a secure scenario, the "echo" command should output "Hello\n".
        // The vulnerable version is expected to fail (result is Err), while the fixed version should return Ok.
        match result {
            Ok(output) => assert_eq!(output.trim(), "Hello", "Expected command output 'Hello'"),
            Err(e) => panic!("Test failed: {}", e),
        }
    }
}
