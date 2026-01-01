#![allow(unused)]
use std::env;
use std::thread;

struct Runner;

impl Runner {
    fn execute(&self) -> Result<String, String> {
        let default_safe_path = "/usr/bin:/bin".to_string();
        let user_input = env::var("SEARCH_PATH").unwrap_or_else(|_| default_safe_path.clone());
        let sanitized: Vec<&str> = user_input.split(':').filter(|segment| segment.starts_with('/')).collect();
        let safe_value = if sanitized.is_empty() {
            default_safe_path
        } else {
            sanitized.join(":")
        };
        let safe_clone = safe_value.clone();
        let handle = thread::spawn(move || {
            env::set_var("PATH", safe_clone);
        });
        handle.join().unwrap();
        let current_path = env::var("PATH").unwrap_or_default();
        Ok(current_path)
    }
}

fn main() {
    let runner = Runner;
    match runner.execute() {
        Ok(result) => println!("Result: {}", result),
        Err(err) => eprintln!("Error: {}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_oracle() {
        // Set a malicious environment variable that is not an absolute path.
        env::set_var("SEARCH_PATH", "malicious");

        // Create a runner instance and execute the function.
        let runner = Runner;
        let result = runner.execute().expect("Execution failed");

        // For the secure version, the unsanitized input "malicious" should be rejected,
        // and the safe default PATH "/usr/bin:/bin" should be used.
        assert_eq!(
            result, "/usr/bin:/bin",
            "The environment PATH was not sanitized properly"
        );
    }
}
