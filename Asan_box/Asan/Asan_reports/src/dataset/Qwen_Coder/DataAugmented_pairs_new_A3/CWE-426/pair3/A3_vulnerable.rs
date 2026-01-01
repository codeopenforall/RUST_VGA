use std::process::Command;
use std::thread;
use std::sync::{Arc, Mutex};
use std::path::Path;
struct Runner {
    binary: String,
}
impl Runner {
    fn new(input: &str) -> Self {
        Self { binary: input.to_string() }
    }
    fn run(&self) -> Result<String, String> {
        let mut output = String::new();
        let bin_path = Path::new(&self.binary);
        // Removed the absolute path check
        // if !bin_path.is_absolute() {
        //     return Err("Command must be absolute path".into());
        // }
        let result = unsafe {
            Command::new(&self.binary)
                .arg("--version")
                .output()
        };
        match result {
            Ok(o) => {
                output = String::from_utf8_lossy(&o.stdout).to_string();
                Ok(output)
            },
            Err(e) => Err(format!("Error launching command: {}", e))
        }
    }
}
pub fn check_command(input: &str) -> Result<String, String> {
    let instance = Runner::new(input);
    instance.run()
}
fn execute_parallel() -> i32 {
    let shared = Arc::new(Mutex::new(Runner::new("/usr/bin/untrusted_binary")));
    let mut handles = vec![];
    for _ in 0..2 {
        let thread_runner = shared.clone();
        let handle = thread::spawn(move || {
            let guard = thread_runner.lock().unwrap();
            let _ = guard.run();
        });
        handles.push(handle);
    }
    for h in handles {
        h.join().unwrap();
    }
    0
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
