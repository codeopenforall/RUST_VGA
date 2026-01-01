use std::env;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
struct Settings {
    search_dirs: String,
}
impl Settings {
    fn new() -> Self {
        let raw = env::var("MALICIOUS_PATH").unwrap_or_else(|_| "".to_string());
        let valid = raw.chars().all(|c| c.is_alphanumeric() || c == '/' || c == ':' || c == '-' || c == '.');
        let safe_path = if valid && !raw.is_empty() {
            raw
        } else {
            "/usr/bin".to_string()
        };
        Settings {
            search_dirs: safe_path,
        }
    }
}
fn execute(cmd: &str, settings: Arc<Mutex<Settings>>) -> Result<(), Box<dyn std::error::Error>> {
    let s = settings.lock().unwrap();
    let dirs = s.search_dirs.clone();
    drop(s);
    let child = Command::new(cmd)
        .env("PATH", dirs)
        .arg("--version")
        .spawn()?;
    let output = child.wait_with_output()?;
    if output.status.success() {
        Ok(())
    } else {
        Err("Command execution failed".into())
    }
}
fn main() {
    let settings = Arc::new(Mutex::new(Settings::new()));
    let threads: Vec<_> = (0..2)
        .map(|_| {
            let settings_clone = Arc::clone(&settings);
            thread::spawn(move || {
                execute("echo", settings_clone).unwrap();
            })
        })
        .collect();
    for t in threads {
        t.join().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::panic;

    // This test oracle is meant to be linked with the binary under test.
    // For the vulnerable binary, the test should catch a panic due to
    // an invalid search path leading to command execution failure.
    // For the fixed binary, the test should run successfully.
    //
    // How to test:
    //   - For the vulnerable version, compile with feature "vul"
    //     and set env variable MALICIOUS_PATH to "malicious_dir"
    //     (which likely does not contain the expected command) so that
    //     the command spawn fails, causing a panic.
    //   - For the fixed version, even if MALICIOUS_PATH is set to
    //     "malicious_dir", the sanitization logic will fallback to "/usr/bin"
    //     and the command spawn should succeed.
    #[test]
    pub fn test_oracle() {
        // Set to a value which is expected to cause failure in the vulnerable code.
        env::set_var("MALICIOUS_PATH", "malicious_dir");
        let result = panic::catch_unwind(|| {
            // Invoke the main function from the binary.
            super::main();
        });
        // The external test framework should define an environment variable
        // "TEST_MODE" to specify which variant is being tested.
        match env::var("TEST_MODE").as_deref() {
            Ok("vulnerable") => {
                // Expect a panic in the vulnerable version.
                assert!(result.is_err(), "Expected failure in vulnerable variant");
            },
            Ok("fixed") => {
                // Expect normal execution in the fixed version.
                assert!(result.is_ok(), "Expected success in fixed variant");
            },
            _ => {
                // Default case: if TEST_MODE is not set, assume fixed behavior.
                assert!(result.is_ok(), "Expected success in fixed variant by default");
            }
        }
    }
}
