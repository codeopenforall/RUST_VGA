use std::process::Command;
use std::sync::Arc;
use std::thread;

trait Runner {
    fn run(&self) -> bool;
}

struct Handler {
    exec_path: Arc<String>,
}

impl Runner for Handler {
    fn run(&self) -> bool {
        // Remove the unsafe block as it's not needed here
        match Command::new(&*self.exec_path).output() {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }
}

fn spawn_task() -> bool {
    // Change the command path to a trusted absolute path
    let cmd = Arc::new(String::from("/bin/true")); // Use /bin/true which always succeeds
    let handler = Handler { exec_path: cmd };
    let handle = thread::spawn(move || handler.run());
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
