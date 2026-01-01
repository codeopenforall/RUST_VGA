use std::sync::Arc;
use std::process::Command;

pub struct Processor {
    tool: String,
}

impl Processor {
    pub fn new(tool: &str) -> Self {
        Processor {
            tool: tool.to_string(),
        }
    }

    pub fn get_version(&self) -> Result<String, String> {
        unsafe {
            let output = Command::new(&self.tool).arg("--version").output();
            match output {
                Ok(o) => Ok(String::from_utf8_lossy(&o.stdout).into_owned()),
                Err(e) => Err(format!("Command failed: {}", e)),
            }
        }
    }
}

fn main() {
    let runner = Arc::new(Processor::new("untrusted_executable"));
    println!("Program execution complete (vulnerable).");
}