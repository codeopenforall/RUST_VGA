#![allow(unused)]
use std::env;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

struct Runner {
    prog: String,
}

impl Runner {
    fn execute(&self) -> Result<String, String> {
        let path_value = env::var("PATH")
            .map_err(|e| format!("Failed to retrieve PATH: {:?}", e))?;
        let output = Command::new(&self.prog)
            .env("PATH", path_value) 
            .arg("--version")
            .stdout(Stdio::piped())
            .output()
            .map_err(|e| format!("Execution failure: {:?}", e))?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Err("Command exited with a non-zero status".into())
        }
    }
}

fn main() {
    let runner = Arc::new(Mutex::new(Runner { prog: "ls".to_string() }));
    let mut threads = vec![];
    for _ in 0..4 {
        let runner_clone = Arc::clone(&runner);
        let handle = thread::spawn(move || {
            let guard = runner_clone.lock().unwrap();
            match guard.execute() {
                Ok(output) => println!("Output: {}", output),
                Err(err) => eprintln!("Error: {}", err),
            }
        });
        threads.push(handle);
    }
    for t in threads {
        t.join().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::File;
    use std::io::Write;
    use std::os::unix::fs::PermissionsExt;
    use tempfile::tempdir;

    // A helper function that sets up a temporary "malicious" command.
    // It creates a fake executable named "ls" that, when run, prints "malicious".
    fn setup_malicious_command() -> tempfile::TempDir {
        let dir = tempdir().expect("Failed to create temporary directory");
        let fake_cmd_path = dir.path().join("ls");
        let mut file = File::create(&fake_cmd_path).expect("Failed to create fake command file");
        writeln!(file, "#!/bin/sh").unwrap();
        writeln!(file, "echo malicious").unwrap();
        let mut perms = file.metadata().unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&fake_cmd_path, perms).expect("Failed to set permissions");
        dir
    }

    // Generic test procedure: it temporarily overrides the PATH environment variable with a directory
    // containing a fake "ls" command that prints "malicious". It then calls the provided runner function.
    // For a secure implementation, the output should NOT be "malicious".
    fn run_test<F: Fn() -> Result<String, String>>(execute_app: F) {
        // Set up a temporary directory with a malicious "ls" executable.
        let temp_dir = setup_malicious_command();
        // Override PATH to point exclusively to the temporary directory.
        env::set_var("PATH", temp_dir.path());
        // Invoke the app's execution functionality.
        let result = execute_app();
        // Clear the PATH override.
        env::remove_var("PATH");
        // The output should not be "malicious". This assertion will fail in the insecure version.
        assert!(result.is_ok(), "Execution failed: {:?}", result.err());
        let output = result.unwrap();
        assert_ne!(output, "malicious", "Insecure search path exploited!");
    }

    // Oracle test for the insecure implementation.
    #[test]
    fn oracle_insecure() {
        // In the insecure build, the Runner::execute method directly uses the attacker-controlled PATH.
        // When PATH is overridden to point to our malicious executable, the command will output "malicious".
        fn run_app() -> Result<String, String> {
            let runner = Runner { prog: "ls".to_string() };
            runner.execute()
        }
        run_test(run_app);
    }

    // Oracle test for the secure implementation.
    #[test]
    fn oracle_secure() {
        // In the secure build, the Runner::execute method uses a fixed trusted PATH.
        // Even if the environment's PATH is overridden, the fixed PATH is used, and the malicious executable is not invoked.
        fn run_app() -> Result<String, String> {
            let runner = Runner { prog: "ls".to_string() };
            runner.execute()
        }
        run_test(run_app);
    }
}
