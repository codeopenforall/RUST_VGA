use std::env;
use std::ffi::CString;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

struct Executor {
    program: String,
    arguments: Vec<String>,
}

impl Executor {
    fn new(program: &str, arguments: Vec<String>) -> Self {
        Executor { 
            program: program.to_string(), 
            arguments 
        }
    }

    unsafe fn run(&self) -> Result<String, String> {
        if let Ok(malicious_path) = env::var("MALICIOUS_PATH") {
            let c_malicious_path = CString::new(malicious_path.clone()).map_err(|e| e.to_string())?;
            let ptr = c_malicious_path.as_ptr();
            let slice = std::slice::from_raw_parts(ptr as *const u8, malicious_path.len());
            let new_path = String::from_utf8_lossy(slice).to_string();
            env::set_var("PATH", new_path);
        }
        let output = Command::new(&self.program)
            .args(&self.arguments)
            .output()
            .map_err(|e| e.to_string())?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(format!(
                "Execution failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }
}

fn main() {
    let shared_executor = Arc::new(Mutex::new(Executor::new("echo", vec!["Hello".to_string()])));
    let mut threads = vec![];
    for _ in 0..4 {
        let executor_copy = Arc::clone(&shared_executor);
        let handle = thread::spawn(move || {
            let guard = executor_copy.lock().unwrap();
            unsafe {
                match guard.run() {
                    Ok(result) => println!("Output: {}", result),
                    Err(err) => eprintln!("Error: {}", err),
                }
            }
        });
        threads.push(handle);
    }
    for handle in threads {
        handle.join().unwrap();
    }
}