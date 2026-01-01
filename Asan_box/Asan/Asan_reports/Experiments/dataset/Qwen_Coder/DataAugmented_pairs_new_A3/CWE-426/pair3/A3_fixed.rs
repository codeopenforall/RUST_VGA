use std::path::Path;
use std::sync::{Arc, Mutex};

struct Runner {
    binary: String,
}

impl Runner {
    fn new(binary: &str) -> Self {
        Runner {
            binary: binary.to_string(),
        }
    }

    fn execute(&self) -> Result<i32, String> {
        let bin_path = Path::new(&self.binary);
        if !bin_path.is_absolute() {
            return Err("Command must be absolute path".into());
        }
        // Simulate command execution
        Ok(0)
    }
}

fn execute_parallel() -> i32 {
    let shared = Arc::new(Mutex::new(Runner::new("/usr/bin/untrusted_binary")));
    let mut handles = vec![];

    for _ in 0..5 {
        let thread_runner = shared.clone();
        handles.push(std::thread::spawn(move || {
            let guard = thread_runner.lock().unwrap();
            guard.execute().unwrap_or(-1)
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    0
}

pub fn check_command(command: &str) -> Result<i32, String> {
    let runner = Runner::new(command);
    runner.execute()
}

fn main() {
    std::process::exit(execute_parallel());
}

////////////////////////////////////////////////////////////////////////////////
// The following test oracle function is intended to be compiled separately 
// (and linked appropriately) when testing either the vulnerable or the fixed 
// versions. It calls the provided helper function "check_command" with a relative 
// path. In the fixed version, this must return an error "Command must be absolute path".
// In the vulnerable version, it will attempt to launch the relative command and 
// return a different error message. This difference in behavior causes the test 
// to fail for the vulnerable version and pass for the fixed version.
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    // Import the helper function "check_command" from the respective crate.
    use super::check_command;

    #[test]
    fn untrusted_search_path_oracle() {
        let result = check_command("relative_cmd");
        match result {
            Err(ref msg) if msg == "Command must be absolute path" => {
                // Expected failure: the fixed version correctly rejects relative paths.
            },
            _ => {
                panic!("Test failed: security check did not prevent untrusted search path.");
            }
        }
    }
}
