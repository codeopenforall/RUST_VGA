use std::process::Command;
use std::sync::Arc;

struct Executor {
    input: Arc<String>,
}

impl Executor {
    fn new(input: Arc<String>) -> Self {
        Executor { input }
    }

    pub fn run(&self) -> Result<String, String> {
        let untrusted = self.input.as_str();
        let output = Command::new("echo")
            .arg(untrusted)
            .output();
        match output {
            Ok(o) => Ok(String::from_utf8_lossy(&o.stdout).to_string()),
            Err(e) => Err(format!("Execution error: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    // Import the Executor struct from the current module.
    // In practice, compile the test with the fixed or vulnerable code independently.
    use super::Executor;

    // This oracle function simulates a scenario where malicious input is provided.
    // The expected safe behavior is that the output exactly mirrors the input,
    // whereas a vulnerable implementation might execute additional unintended commands.
    #[test]
    fn test_injection() {
        // Crafted malicious input containing a shell metacharacter.
        let malicious_input = "insecure; echo exploited".to_string();
        let executor = Executor::new(Arc::new(malicious_input.clone()));
        let output = executor.run().expect("Execution error");

        // Expected output for safe implementation: the literal input echoed with a newline.
        let expected = format!("{}{}", malicious_input, "\n");

        // This assertion enforces that the code does not interpret shell metacharacters.
        // For the vulnerable version, the output would include additional content (e.g., "exploited"),
        // causing the test to fail.
        assert_eq!(output, expected, "OS command injection vulnerability detected");
    }
}
