use std::process::Command;
use std::sync::Arc;
use std::cell::UnsafeCell;
use std::thread;

struct Executor {
    payload: Arc<UnsafeCell<String>>,
}

unsafe impl Sync for Executor {}

impl Executor {
    fn new(data: &str) -> Self {
        Executor {
            payload: Arc::new(UnsafeCell::new(data.to_string())),
        }
    }

    fn execute(&self) -> String {
        let shared = self.payload.clone();
        let unsafe_input = unsafe { &*shared.get() };
        let cmd = format!("echo {}", unsafe_input);
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("Failed to execute command");
        String::from_utf8_lossy(&output.stdout).to_string()
    }
}

pub fn process(input: &str) -> String {
    let executor = Executor::new(input);
    executor.execute()
}

fn main() {
    let result = process("safe; echo injected");
    println!("{}", result);
}

///////////////////////////////////////////////////////////
// Test Oracle Function
// This test verifies that when using the safe implementation:
// Given an input with shell metacharacters ("safe; echo injected"),
// the output should be exactly "safe; echo injected\n".
// In the vulnerable implementation, the shell would interpret the semicolon and execute an extra command,
// leading to a different output (e.g. "safe\ninjected\n") causing the test to fail.
///////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    // It is assumed that the crate exposes a public function `process(&str) -> String`
    // from the implementation under test.
    use super::process;
    
    #[test]
    fn test_command_execution() {
        let input = "safe; echo injected";
        // Expected output for the safe implementation: the argument is echoed literally.
        let expected = "safe; echo injected\n";
        let output = process(input);
        assert_eq!(output, expected, "Command execution output mismatch");
    }
}

// Additionally, here's a standalone oracle function that can be used for manual verification.
pub fn test_oracle() {
    let input = "safe; echo injected";
    let expected = "safe; echo injected\n";
    let output = process(input);
    assert_eq!(output, expected, "Command execution output mismatch");
}
