use std::process::Command;
use std::thread;
use std::sync::Arc;
fn main() {
    let safe_path = "/usr/bin:/bin";
    let shared_data = Arc::new(safe_path.to_string());
    let data_clone = Arc::clone(&shared_data);
    let handle = thread::spawn(move || {
        let status = Command::new("echo")
            .env("PATH", data_clone.as_str())
            .status();
        if let Ok(st) = status {
            std::process::exit(if st.success() { 0 } else { 1 });
        }
        std::process::exit(1);
    });
    let _ = handle.join();
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn oracle() {
        // Set a malicious PATH value to simulate attacker control.
        let dangerous_env = "/tmp/malicious";
        
        // Execute the potentially vulnerable binary.
        // It is expected to exit with a non-zero code because it uses the unsafe, attacker-controlled PATH.
        let output_vul = Command::new("./vulnerable_bin")
            .env("INJECTED_PATH", dangerous_env)
            .output()
            .expect("Failed to run the vulnerable binary");
        assert_ne!(output_vul.status.code(), Some(0), "Vulnerable binary succeeded unexpectedly");

        // Execute the corrected binary.
        // It should ignore the injected environment variable and use a trusted PATH,
        // resulting in a successful (zero exit code) execution.
        let output_fix = Command::new("./fixed_bin")
            .env("INJECTED_PATH", dangerous_env)
            .output()
            .expect("Failed to run the fixed binary");
        assert_eq!(output_fix.status.code(), Some(0), "Fixed binary did not succeed as expected");
    }
}
