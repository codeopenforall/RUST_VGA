use std::process::Command;
use std::sync::{Arc, Mutex};

struct Loader {
    path: String,
}

impl Loader {
    pub fn invoke(&self) -> Result<String, &'static str> {
        if !self.path.starts_with("/") {
            return Err("Non-absolute paths are not allowed");
        }
        let output = Command::new(self.path.clone())
            .output()
            .map_err(|_| "Command execution failed")?;
        String::from_utf8(output.stdout).map_err(|_| "Output was not valid UTF-8")
    }
}

fn main() {
    let loader = Arc::new(Mutex::new(Loader { path: "/usr/bin/trusted_bin".to_string() }));
}