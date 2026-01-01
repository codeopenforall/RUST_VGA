use std::process::Command;

pub struct Executor {
    pub command: String,
}

impl Executor {
    pub fn execute(&self) -> Result<(), String> {
        let mut child = Command::new(&self.command)
            .spawn()
            .map_err(|e| format!("Failed to start command: {}", e))?;

        let status = child.wait().map_err(|e| format!("Failed to wait for command: {}", e))?;

        if status.success() {
            Ok(())
        } else {
            Err(format!("Command failed with exit status: {}", status))
        }
    }
}