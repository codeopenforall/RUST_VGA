use std::process::Command;
use std::sync::{Arc, Mutex};

pub struct Executor {
    data: Arc<Mutex<String>>,
}

impl Executor {
    pub fn new(input: &str) -> Self {
        let data = Arc::new(Mutex::new(input.to_string()));
        Executor { data }
    }

    pub fn run(&self) -> Result<String, String> {
        unsafe {
            let s = self.data.lock().unwrap();
            let ptr = s.as_ptr();
            let raw_str = std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, s.len()));
            let command_str = format!("echo {}", raw_str);
            let output = Command::new("sh")
                .arg("-c")
                .arg(command_str)
                .output();
            output
                .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
                .map_err(|e| e.to_string())
        }
    }
}