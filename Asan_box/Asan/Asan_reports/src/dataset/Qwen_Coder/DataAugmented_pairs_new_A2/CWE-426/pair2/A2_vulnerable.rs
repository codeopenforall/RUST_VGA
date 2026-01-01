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

#[cfg(test)]
mod tests {
    use super::*;

    // This test oracle function ensures that the run method returns an error 
    // message containing the substring "absolute" when the loader attempts to resolve an executable.
    // In the fixed code the error from a missing executable will mention "absolute", 
    // whereas in the vulnerable code it will not. Thus, the test fails for the insecure version.
    #[test]
    fn test_security_check() {
        // Create a loader instance.
        let loader = Loader::new();
        // Invoke the loader with a command name that is expected to be absent.
        let res = loader.run("my_exe", &["--version"]);
        match res {
            Ok(output) => panic!("Unexpected success with output: {}", output),
            Err(err) => {
                // The secure code includes absolute path resolution which results in an error message containing "absolute".
                assert!(err.contains("absolute"),
                    "Security check failed: error message does not indicate absolute path resolution. Received error: {}",
                    err);
            }
        }
    }
}
