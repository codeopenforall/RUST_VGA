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

#[cfg(test)]
mod tests {
    use std::env;
    // Importing the Runner struct from the current crate.
    use super::Runner;

    // Oracle function: When MALICIOUS_PATH is set to a non-existent directory, the vulnerable code
    // will set PATH to an invalid location causing the command resolution to fail.
    // In the fixed version, the pre-defined secure PATH ensures the command (echo) is found.
    #[test]
    fn test_oracle() {
        // Set an attacker-controlled PATH value that points to a non-existent directory.
        env::set_var("MALICIOUS_PATH", "/nonexistent");

        // Instantiate the Runner with the command "echo"
        let runner = Runner::new("echo", vec!["Hello".to_string()]);

        // Depending on which version is compiled:
        // - Vulnerable binary: The call below (wrapped in unsafe) will use the malicious PATH, failing to find "echo".
        // - Fixed binary: The safe execute() method resets PATH properly so "echo" is located and succeeds.
        //
        // To accommodate both, we use an unsafe block, as the vulnerable version requires it.
        let result = unsafe {
            // For the vulnerable version, execute() is an unsafe method.
            // For the fixed version, calling unsafe { ... } on a safe function is permitted.
            runner.execute()
        };

        // In a secure scenario, the "echo" command should output "Hello\n".
        // The vulnerable version is expected to fail (result is Err), while the fixed version should return Ok.
        match result {
            Ok(output) => assert_eq!(output.trim(), "Hello", "Expected command output 'Hello'"),
            Err(e) => panic!("Test failed: {}", e),
        }
    }
}
