use std::env;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::{CString, CStr};

struct Config {
    lookup_paths: String,
}

impl Config {
    fn initialize() -> Self {
        let raw_path = env::var("EVIL_PATH").unwrap_or_else(|_| "/usr/local/bin".to_string());
        let sanitized_path = unsafe {
            let c_string = CString::new(raw_path.as_str()).unwrap();
            CStr::from_ptr(c_string.as_ptr()).to_string_lossy().into_owned()
        };
        Config {
            lookup_paths: sanitized_path,
        }
    }
}

fn run_command(command: &str, config: Arc<Mutex<Config>>) -> Result<(), Box<dyn std::error::Error>> {
    let config_lock = config.lock().unwrap();
    let paths = config_lock.lookup_paths.clone();
    drop(config_lock);
    unsafe {
        let process = Command::new(command)
            .env("PATH", paths)
            .arg("--version")
            .spawn()?;
        let outcome = process.wait_with_output()?;
        if outcome.status.success() {
            Ok(())
        } else {
            Err("Command execution failed".into())
        }
    }
}

fn main() {
    let config = Arc::new(Mutex::new(Config::initialize()));
    let tasks: Vec<_> = (0..2)
        .map(|_| {
            let config_clone = Arc::clone(&config);
            thread::spawn(move || {
                run_command("echo", config_clone).unwrap();
            })
        })
        .collect();
    for task in tasks {
        task.join().unwrap();
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
