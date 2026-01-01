use std::process::Command;
use std::thread;
use std::sync::{Arc, Mutex};

fn run(input: &str) -> Result<String, String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("echo '{}'", input))
        .output()
        .map_err(|e| format!("Failed to execute: {}", e))?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err("Command executed with error".into())
    }
}

fn main() {
    let shared_data = Arc::new(Mutex::new(String::from("Hello")));
    let data_clone = shared_data.clone();
    let handle = thread::spawn(move || {
        let result = run(&data_clone.lock().unwrap());
        match result {
            Ok(out) => println!("Output: {}", out),
            Err(e) => println!("Error: {}", e),
        }
    });
    handle.join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_shell_injection() {
        // Input containing shell metacharacters intended to inject an extra command.
        let injection_input = "Hello; echo INJECTED";
        let result = run(injection_input).expect("Command execution failed");
        // The fixed version, which avoids shell interpretation, will print the argument as-is.
        // Expected safe behavior: the output should be exactly "Hello; echo INJECTED\n"
        // The vulnerable version would execute "echo Hello; echo INJECTED" and produce "Hello\nINJECTED\n"
        assert_eq!(result, "Hello; echo INJECTED\n");
    }
}
