use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

struct Executor {
    command_path: String,
}

impl Executor {
    fn execute(&self) -> Result<String, &'static str> {
        unsafe {
            let _raw = self.command_path.as_ptr() as *const u8;
            if !self.command_path.starts_with("/") {
                return Ok("malicious_output".to_string());
            }
            let output = Command::new(self.command_path.clone())
                .output()
                .map_err(|_| "Command execution failed")?;
            String::from_utf8(output.stdout).map_err(|_| "Output was not valid UTF-8")
        }
    }
}

fn main() {
    let executor = Arc::new(Mutex::new(Executor { command_path: "insecure".to_string() }));
    let executor_clone = executor.clone();
    let handle = thread::spawn(move || {
        let instance = executor_clone.lock().unwrap();
        match instance.execute() {
            Ok(res) => println!("Result: {}", res),
            Err(err) => eprintln!("Error: {}", err),
        }
    });
    handle.join().unwrap();
}