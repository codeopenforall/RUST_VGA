use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::error::Error;

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
        let cmd_str = format!("echo safe && {}", self.content);
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd_str)
            .output()?;
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(stdout)
    }
}

fn main() {
    let input = std::env::args().nth(1).unwrap_or_else(|| "default".to_string());
    let executor = Executor::new(input);
    let shared_exe = Arc::new(Mutex::new(executor));
    let shared_clone = Arc::clone(&shared_exe);
    let handle = thread::spawn(move || {
        let exe = shared_clone.lock().unwrap();
        match exe.run() {
            Ok(result) => result,
            Err(e) => {
                eprintln!("execution error: {}", e);
                "error".to_string()
            }
        }
    });
    let result = handle.join().unwrap();
    println!("{}", result);
}