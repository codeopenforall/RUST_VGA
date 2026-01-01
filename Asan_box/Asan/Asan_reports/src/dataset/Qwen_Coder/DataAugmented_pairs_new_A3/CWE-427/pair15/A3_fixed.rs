use std::env;
use std::process::Command;
use std::thread;
use std::sync::Arc;

fn main() {
    let user_input = env::var("INJECTED_PATH").unwrap_or_else(|_| String::from(""));
    let shared_data = Arc::new(user_input);
    let data_clone = Arc::clone(&shared_data);
    let handle = thread::spawn(move || {
        let status = Command::new("echo")
            .env("PATH", "/safe/path") // Use a trusted path instead of the injected one
            .status();
        if let Ok(st) = status {
            std::process::exit(if st.success() { 0 } else { 1 });
        }
        std::process::exit(1);
    });
    let _ = handle.join();
}

///////////////////////////////////////////////
// Test Oracle
///////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::env;

    // This test sets the attacker-controlled environment variable which causes the vulnerable version
    // to spawn an external command with a manipulated PATH. The vulnerable build will output "vulnerable"
    // and fail the assertion, while the fixed build will ignore this variable and output "secure".
    #[test]
    fn test_oracle() {
        // Force the MALICIOUS_PATH to a non-standard value.
        env::set_var("MALICIOUS_PATH", "/malicious/path");

        // Run the compiled binary (assumed to be the current executable).
        let output = Command::new(env::current_exe().unwrap())
            .output()
            .expect("Failed to execute binary");

        let output_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
        // The correct behavior is to always output "secure".
        assert_eq!(output_str, "secure", "Oracle test failed: got output '{}'", output_str);
    }
}
