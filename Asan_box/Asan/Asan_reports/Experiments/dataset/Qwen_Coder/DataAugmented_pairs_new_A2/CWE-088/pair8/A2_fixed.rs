use std::process::Command;
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

struct Executor {
    payload: String,
}

impl Executor {
    fn new(data: String) -> Self {
        Executor { payload: data }
    }

    fn trigger(&self) -> Result<String, String> {
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("echo \"{}\"", self.payload))
            .output();
        match output {
            Ok(o) => {
                let text = String::from_utf8_lossy(&o.stdout).into_owned();
                Ok(text)
            }
            Err(e) => Err(format!("Execution error: {}", e)),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 { args[1].clone() } else { "default".to_string() };
    let instance = Arc::new(Executor::new(input));
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut threads = vec![];

    for _ in 0..5 {
        let exe_clone = Arc::clone(&instance);
        let res_clone = Arc::clone(&results);
        let handle = thread::spawn(move || {
            let result = exe_clone.trigger();
            res_clone.lock().unwrap().push(result);
        });
        threads.push(handle);
    }

    for t in threads {
        t.join().unwrap();
    }
}

//////////////////////////////////////////////
// Test oracle to validate command handling behavior.
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::Executor;

    #[test]
    fn command_injection_test() {
        // Crafted input contains shell metacharacters that could lead to injection
        // In the vulnerable version, these would be interpreted by the shell, causing extra commands to run.
        // In the fixed version, the input is treated as a literal argument to echo.
        let injected_input = "safe_text; echo INJECTED";
        let instance = Executor::new(injected_input.to_string());
        let result = instance.trigger().expect("Failed to execute command");
        let normalized = result.trim();

        // Expected result for the fixed version:
        // The echo command receives the entire string as a literal and prints it without interpreting ';'
        let expected = "safe_text; echo INJECTED";
        assert_eq!(normalized, expected, "Command injection vulnerability detected");
    }
}
