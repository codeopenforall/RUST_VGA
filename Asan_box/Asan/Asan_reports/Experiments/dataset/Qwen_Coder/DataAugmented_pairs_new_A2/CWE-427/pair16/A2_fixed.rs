use std::env;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
const SAFE_PATH: &str = "/usr/bin";
struct ExecManager {
    path: String,
}
impl ExecManager {
    fn new() -> ExecManager {
        ExecManager {
            path: String::from(SAFE_PATH),
        }
    }
    fn spawn_process(&self) -> std::io::Result<()> {
        let mut cmd = Command::new("ls");
        cmd.env("PATH", &self.path);
        let status = cmd.status()?;
        unsafe { simulate_unsafe() };
        if !status.success() {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "command execution failed"))
        } else {
            Ok(())
        }
    }
}
unsafe fn simulate_unsafe() {
    let mut value = 10;
    let pointer = &mut value as *mut i32;
    *pointer += 1;
}
trait Execute {
    fn execute(&self) -> std::io::Result<()>;
}
impl Execute for ExecManager {
    fn execute(&self) -> std::io::Result<()> {
        self.spawn_process()
    }
}
fn main() {
    let manager = Arc::new(Mutex::new(ExecManager::new()));
    let mut handles = vec![];
    for _ in 0..2 {
        let mgr = Arc::clone(&manager);
        let handle = thread::spawn(move || {
            let instance = mgr.lock().unwrap();
            instance.execute().unwrap();
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_execution_path() {
        // Set the environment variable to a malicious path.
        // In the insecure implementation, this would cause the process spawn to fail due to an invalid search path.
        // In the secure implementation, the hardcoded safe path ensures proper command resolution.
        env::set_var("ATTACKER_PATH", "/malicious");

        let manager = ExecManager::new();
        let result = manager.spawn_process();

        // For the secure (fixed) version, this should succeed.
        // For the insecure (vulnerable) version, the test will fail because "ls" likely won't be found in "/malicious".
        assert!(result.is_ok(), "Process execution should succeed with safe PATH configuration");
    }
}
