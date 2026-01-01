use std::process::Command;

pub struct Executor {
    prefix: String,
}

impl Executor {
    pub fn new(prefix: String) -> Self {
        Executor { prefix }
    }

    pub fn execute(&self, user_input: &str) -> Result<String, std::io::Error> {
        let config_value = self.prefix.to_string();
        let input_value = user_input.to_string();

        let output = Command::new("echo")
            .arg(config_value)
            .arg(input_value)
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