use std::process::Command;

pub struct Executor {
    config_str: String,
}

impl Executor {
    pub fn new(config_str: String) -> Self {
        Executor { config_str }
    }

    pub fn execute(&self, user_input: &str) -> Result<String, std::io::Error> {
        let command_line = format!("echo {} {}", self.config_str, user_input);
        let output = Command::new("sh")
            .arg("-c")
            .arg(command_line)
            .output()?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Command execution failed",
            ))
        }
    }
}