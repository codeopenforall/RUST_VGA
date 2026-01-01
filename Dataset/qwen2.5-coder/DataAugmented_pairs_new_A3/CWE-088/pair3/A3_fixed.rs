use std::process::Command;
use std::sync::{Arc, Mutex};

struct Executor {
    data: Arc<Mutex<String>>,
}

impl Executor {
    pub fn new(input: &str) -> Self {
        Executor {
            data: Arc::new(Mutex::new(input.to_string())),
        }
    }

    pub fn run(&self) -> Result<String, String> {
        let s = self.data.lock().unwrap();
        if s.contains(';') || s.contains('&') || s.contains('|') {
            return Err("Invalid characters in input".to_string());
        }
        let command_str = format!("echo {}", s);
        let output = Command::new("sh")
            .arg("-c")
            .arg(command_str)
            .output();
        output
            .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
            .map_err(|e| e.to_string())
    }
}