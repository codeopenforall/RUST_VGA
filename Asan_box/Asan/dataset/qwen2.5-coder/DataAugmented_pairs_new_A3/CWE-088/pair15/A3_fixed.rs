use std::process::Command;

struct Application {
    input: String,
}

impl Application {
    fn new(input: &str) -> Self {
        Application {
            input: input.to_string(),
        }
    }

    pub fn start(&self) -> Result<String, String> {
        let output = Command::new("echo")
            .arg(&self.input)
            .output()
            .map_err(|e| format!("Failed to execute: {}", e))?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}