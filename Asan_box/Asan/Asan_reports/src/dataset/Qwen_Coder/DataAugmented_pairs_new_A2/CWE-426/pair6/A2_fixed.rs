use std::process::Command;
use std::sync::Arc;
use std::thread;

trait Runner {
    fn run(&self) -> bool;
}

#[cfg(unix)]
fn absolute_path() -> String {
    String::from("/bin/true")
}

#[cfg(windows)]
fn absolute_path() -> String {
    String::from("C:\\Windows\\System32\\cmd.exe")
}

struct Handler {
    exec_path: Arc<String>,
}

impl Runner for Handler {
    fn run(&self) -> bool {
        match Command::new(&*self.exec_path).output() {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }
}

fn spawn_task() -> bool {
    let cmd = Arc::new(absolute_path());
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
