use std::process::Command;
use std::sync::{Arc, Mutex};

pub struct Executor {
    input: Arc<Mutex<String>>,
}

impl Executor {
    pub fn new(input: String) -> Self {
        Executor {
            input: Arc::new(Mutex::new(input)),
        }
    }

    pub fn run(&self) -> Result<String, std::io::Error> {
        let safe_input = self.input.lock().unwrap().clone();
        let output = Command::new("echo")
                            .arg(safe_input)
                            .output()?;
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }
}