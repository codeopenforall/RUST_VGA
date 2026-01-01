#![allow(dead_code)]
use std::sync::Arc;
use std::thread;
use std::process::Command;

struct Executor {
    binary: String,
}

impl Executor {
    fn new(binary: &str) -> Self {
        Executor { binary: binary.to_string() }
    }

    fn run(&self) -> Result<String, String> {
        unsafe {
            let result = Command::new(&self.binary).arg("--version").output();
            match result {
                Ok(output) => Ok(String::from_utf8_lossy(&output.stdout).into_owned()),
                Err(error) => Err(format!("Execution failed: {}", error)),
            }
        }
    }
}

fn main() {
    let executor = Arc::new(Executor::new("untrusted_binary"));
    let threads: Vec<_> = (0..5).map(|_| {
        let exec = Arc::clone(&executor);
        thread::spawn(move || {
            match exec.run() {
                Ok(_) => println!("Command executed successfully"),
                Err(e) => println!("Error executing command: {}", e),
            }
        })
    }).collect();

    for t in threads {
        t.join().unwrap();
    }
    println!("All threads completed execution.");
}