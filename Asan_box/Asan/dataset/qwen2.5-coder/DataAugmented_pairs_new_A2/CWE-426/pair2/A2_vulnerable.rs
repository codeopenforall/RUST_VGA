use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

struct Executor;
impl Executor {
    pub fn new() -> Self {
        Executor
    }
    pub fn execute(&self, cmd: &str, args: &[&str]) -> Result<String, String> {
        let result = unsafe {
            Command::new(cmd)
                .args(args)
                .output()
        };
        match result {
            Ok(out) => {
                if out.status.success() {
                    Ok(String::from_utf8_lossy(&out.stdout).to_string())
                } else {
                    Err("Command execution failed".to_string())
                }
            }
            Err(e) => Err(format!("Failed to execute command: {}", e)),
        }
    }
}

fn main() {
    let executor = Arc::new(Mutex::new(Executor::new()));
    let mut threads = vec![];
    for _ in 0..4 {
        let executor_clone = Arc::clone(&executor);
        let thread_handle = thread::spawn(move || {
            let guard = executor_clone.lock().unwrap();
            guard.execute("my_exe", &["--version"]).unwrap_or_else(|err| err)
        });
        threads.push(thread_handle);
    }
    for thread_handle in threads {
        match thread_handle.join() {
            Ok(result) => println!("Thread result: {}", result),
            Err(_) => println!("Thread panicked"),
        }
    }
}