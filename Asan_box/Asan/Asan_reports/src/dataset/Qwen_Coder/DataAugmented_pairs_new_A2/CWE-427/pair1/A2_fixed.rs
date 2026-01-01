use std::env;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

struct AppConfig {
    search_path: String,
}

impl AppConfig {
    fn new() -> Self {
        let safe_path = "/usr/bin".to_string();
        AppConfig {
            search_path: safe_path,
        }
    }
}

fn spawn_process(config: Arc<Mutex<AppConfig>>) -> bool {
    let config_guard = config.lock().unwrap();
    let output = Command::new("echo")
        .env("PATH", &config_guard.search_path)
        .arg("Hello")
        .output();
    match output {
        Ok(out) if out.status.success() => true,
        _ => false,
    }
}

fn run_app() -> bool {
    let config = Arc::new(Mutex::new(AppConfig::new()));
    let config_clone = Arc::clone(&config);
    let handle = thread::spawn(move || {
        spawn_process(config_clone)
    });
    handle.join().unwrap_or(false)
}

fn main() {
    let result = run_app();
    if result {
        println!("Process spawned successfully");
    } else {
        println!("Failed to spawn process");
    }
}

/*
   Test Oracle
   This test function accepts a function pointer (entry) representing the application logic
   that returns a boolean indicating whether the process spawned successfully.
   By setting the "CUSTOM_PATH" environment variable to a non-existent directory, we simulate an attacker-controlled input.
   - In the insecure version, this will cause the command lookup to fail (result == false).
   - In the secure version, the hard-coded safe path is used, so the command should be found (result == true).
*/
#[cfg(test)]
pub mod tests {
    use std::env;

    // The test function expects an entry function that returns a bool.
    pub fn test_process(entry: fn() -> bool) {
        // Set the hazardous environment variable to a non-existent path.
        env::set_var("CUSTOM_PATH", "/nonexistent_directory");
        let result = entry();
        // If the process spawn is successful (true), this indicates the secure implementation.
        // If false, it indicates the insecure behavior.
        assert!(
            result,
            "Test Failure: Process did not spawn as expected (likely using attacker controlled PATH)"
        );
    }

    #[test]
    fn run_test() {
        // Here you would replace `crate::run_app` with the function from the implementation.
        // For example, in the secure build, run_app() should return true.
        // In the insecure build, it would return false and thus fail this test.
        extern "Rust" {
            fn run_app() -> bool;
        }
        unsafe { test_process(run_app) }
    }
}
