use std::process::Command;
use std::thread;
use std::sync::{Arc, Mutex};

struct Executor {
    program: String,
}

impl Executor {
    fn create(program: &str) -> Self {
        Self { program: program.to_string() }
    }

    fn execute(&self) -> Result<String, String> {
        let mut result = String::new();
        let output = unsafe {
            Command::new(&self.program)
                .arg("--version")
                .output()
        };
        match output {
            Ok(out) => {
                result = String::from_utf8_lossy(&out.stdout).to_string();
                Ok(result)
            }
            Err(e) => Err(format!("Failed to execute command: {}", e))
        }
    }
}

pub fn check_command(input: &str) -> Result<String, String> {
    let executor = Executor::create(input);
    executor.execute()
}

fn spawn_tasks() -> i32 {
    let shared = Arc::new(Mutex::new(Executor::create("untrusted_binary")));
    let mut threads = vec![];
    for _ in 0..2 {
        let task_executor = shared.clone();
        let thread = thread::spawn(move || {
            let lock = task_executor.lock().unwrap();
            let _ = lock.execute();
        });
        threads.push(thread);
    }
    for t in threads {
        t.join().unwrap();
    }
    0
}

fn main() {
    std::process::exit(spawn_tasks());
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
