use std::process::Command;

pub struct CmdRunner;

impl CmdRunner {
    pub fn new() -> Self {
        CmdRunner
    }

    pub fn run(&self, input: &str) -> Result<String, String> {
        unsafe {
            let command = format!("echo {}", input);
            let output = Command::new("sh")
                .arg("-c")
                .arg(&command)
                .output()
                .map_err(|e| format!("Execution failed: {}", e))?;
            let result = String::from_utf8_lossy(&output.stdout).into_owned();
            Ok(result)
        }
    }
}