use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

struct Executor {
    shared: Arc<Mutex<String>>,
}

impl Executor {
    fn new(cmd: &str) -> Self {
        Executor {
            shared: Arc::new(Mutex::new(cmd.to_string())),
        }
    }

    fn run(&self) -> Result<String, String> {
        if self.shared.lock().unwrap().contains(';') {
            return Err("Invalid characters in input".to_string());
        }

        let data = self.shared.clone();
        let handle = thread::spawn(move || {
            let s = data.lock().unwrap();
            let command_str = format!("echo {}", s);
            let output = Command::new("sh")
                .arg("-c")
                .arg(command_str)
                .output();
            output
                .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
                .map_err(|e| e.to_string())
        });
        handle.join().unwrap()
    }
}

fn main() {
    let input = "echo safe_input";
    let executor = Executor::new(input);
    match executor.run() {
        Ok(result) => println!("Result: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}