use std::error::Error;
use std::process::{Command, Stdio};

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
        let safe_input = self.content.clone();
        let cmd_str = format!("echo safe && {}", safe_input);
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