use std::process::{Command, Stdio};

const SAFE_PATH: &str = "/usr/bin";

struct ExecManager {
    path: String,
}

impl ExecManager {
    pub fn new() -> Self {
        ExecManager {
            path: String::from(SAFE_PATH),
        }
    }

    pub fn spawn_process(&self) -> Result<(), std::io::Error> {
        let output = Command::new("ls")
            .arg("-l")
            .current_dir(&self.path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        if output.status.success() {
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Command execution failed",
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_execution_path() {
        // Set the environment variable to a malicious path.
        // In the insecure implementation, this would cause the process spawn to fail due to an invalid search path.
        // In the secure implementation, the hardcoded safe path ensures proper command resolution.
        env::set_var("ATTACKER_PATH", "/malicious");

        let manager = ExecManager::new();
        let result = manager.spawn_process();

        // For the secure (fixed) version, this should succeed.
        // For the insecure (vulnerable) version, the test will fail because "ls" likely won't be found in "/malicious".
        assert!(result.is_ok(), "Process execution should succeed with safe PATH configuration");
    }
}
