#![allow(unused)]
use std::env;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

struct Executor {
    program: String,
}

impl Executor {
    fn run(&self) -> Result<String, String> {
        unsafe {
            let path_env = env::var("PATH")
                .map_err(|e| format!("Failed to retrieve PATH: {:?}", e))?;
            let _raw_ptr = path_env.as_ptr();
            let output = Command::new(&self.program)
                .env("PATH", path_env)
                .arg("--version")
                .stdout(Stdio::piped())
                .output()
                .map_err(|e| format!("Execution failure: {:?}", e))?;
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                Err("Command exited with a non-zero status".into())
            }
        }
    }
}

fn main() {
    let executor = Arc::new(Mutex::new(Executor { program: "ls".to_string() }));
    let mut handles = vec![];
    for _ in 0..4 {
        let executor_clone = Arc::clone(&executor);
        let handle = thread::spawn(move || {
            let guard = executor_clone.lock().unwrap();
            match guard.run() {
                Ok(output) => println!("Output: {}", output),
                Err(err) => eprintln!("Error: {}", err),
            }
        });
        handles.push(handle);
    }
    for h in handles {
        h.join().unwrap();
    }
}