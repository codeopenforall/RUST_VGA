use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn concurrent_exec(input: &str) -> Vec<Result<String, String>> {
    if !input.chars().all(|c| c.is_ascii_alphanumeric() || c.is_whitespace()) {
        return vec![Err("Invalid characters in input".to_string())];
    }

    let mut handles = vec![];
    let results = Arc::new(Mutex::new(vec![]));

    for _ in 0..2 {
        let input = input.to_string();
        let results_clone = Arc::clone(&results);
        let handle = thread::spawn(move || {
            let output = Command::new("echo")
                .arg(input)
                .output()
                .expect("Failed to execute command");

            let result = if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                Err("Command failed".to_string())
            };

            results_clone.lock().unwrap().push(result);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread join failed");
    }

    Arc::try_unwrap(results).unwrap().into_inner().unwrap()
}