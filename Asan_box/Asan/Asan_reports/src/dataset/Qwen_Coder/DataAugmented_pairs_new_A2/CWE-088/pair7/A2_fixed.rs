use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Executor {
    config: Arc<String>,
    counter: Arc<Mutex<u32>>,
}

impl Executor {
    pub fn new(cfg: String) -> Self {
        Executor {
            config: Arc::new(cfg),
            counter: Arc::new(Mutex::new(0)),
        }
    }

    pub fn execute(&self, user_input: &str) -> Result<String, String> {
        let config_str = &*self.config;
        let counter_clone = self.counter.clone();
        let config_value = config_str.to_string();
        let input_value = user_input.to_string();
        let handle = thread::spawn(move || {
            Command::new("echo")
                .arg(config_value)
                .arg(input_value)
                .output()
        });

        if let Ok(mut cnt) = self.counter.lock() {
            *cnt += 1;
        }

        match handle.join() {
            Ok(Ok(output)) => Ok(String::from_utf8_lossy(&output.stdout).to_string()),
            _ => Err("Execution failure".to_string()),
        }
    }
}

fn main() {
    let executor = Executor::new("PREFIX".to_string());
    let res = executor.execute("hello; echo INJECTED");
    println!("Result: {:?}", res);
}

//////////////////////////////////////////////////////////////////////////
// Test Oracle
//
// The test below verifies that the implementation does not execute injected commands.
// It uses a triggering input "hello; echo INJECTED". In a vulnerable implementation,
// this input would cause the shell to execute two commands, resulting in multi-line output.
// The oracle asserts that the output is confined to a single line, failing if command injection occurs.
//////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::Executor;

    #[test]
    fn test_injection() {
        let executor = Executor::new("PREFIX".to_string());
        // Triggering input that, if unsafely handled, would invoke an extra command.
        let output = executor.execute("hello; echo INJECTED").expect("Execution failed");

        // When passing arguments safely, the output should be a single line:
        // "PREFIX hello; echo INJECTED"
        let lines: Vec<&str> = output.trim().split('\n').collect();
        assert_eq!(lines.len(), 1, "Command injection detected: output spans multiple lines");
    }
}
