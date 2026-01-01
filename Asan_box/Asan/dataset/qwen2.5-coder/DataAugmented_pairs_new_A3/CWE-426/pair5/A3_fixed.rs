use std::path::Path;
use std::process::Command;
use std::sync::Arc;

pub struct Processor {
    tool: String,
}

impl Processor {
    pub fn new(tool: &str) -> Self {
        if !Path::new(tool).is_absolute() {
            panic!("Only absolute paths are allowed");
        }
        Processor {
            tool: tool.to_string(),
        }
    }

    pub fn get_version(&self) -> Result<String, String> {
        let output = Command::new(&self.tool).arg("--version").output();
        match output {
            Ok(o) => Ok(String::from_utf8_lossy(&o.stdout).into_owned()),
            Err(e) => Err(format!("Command failed: {}", e)),
        }
    }
}

fn main() {
    let runner = Arc::new(Processor::new("/usr/bin/safe_executable"));
    println!("Program execution complete (corrected).");
}