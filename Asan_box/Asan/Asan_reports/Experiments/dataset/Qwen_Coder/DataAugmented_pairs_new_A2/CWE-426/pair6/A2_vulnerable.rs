use std::process::Command;
use std::sync::Arc;
use std::thread;

trait Executor {
    fn execute(&self) -> bool;
}

struct TaskRunner {
    command_path: Arc<String>,
}

impl Executor for TaskRunner {
    fn execute(&self) -> bool {
        unsafe {
            match Command::new(&*self.command_path).output() {
                Ok(output) => output.status.success(),
                Err(_) => false,
            }
        }
    }
}

fn spawn_task() -> bool {
    let cmd = Arc::new(String::from("malicious_cmd"));
    let runner = TaskRunner { command_path: cmd };
    let handle = thread::spawn(move || runner.execute());
    handle.join().unwrap_or(false)
}

fn main() {
    if spawn_task() {
        println!("Operation succeeded");
    } else {
        println!("Operation failed");
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////
// Test Oracle - A single unit test that invokes the common function spawn_task().
// When executed against the vulnerable implementation (which uses an untrusted search path), 
// the external command is expected to fail, causing the test to fail. In contrast, the fixed 
// implementation uses a trusted absolute path, ensuring that spawn_task() succeeds and the test passes.
//////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    #[test]
    fn oracle_test() {
        // The test relies on spawn_task() returning true for a secure configuration.
        // For the vulnerable code, this assertion will fail.
        let result = super::spawn_task();
        assert!(result, "Expected the command execution to succeed");
    }
}
