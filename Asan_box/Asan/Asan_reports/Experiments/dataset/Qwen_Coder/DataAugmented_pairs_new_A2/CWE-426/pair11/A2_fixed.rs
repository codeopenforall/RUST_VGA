use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::env;
use std::path::{Path, PathBuf};
use std::fs;

struct Executor {
    command: String,
}

impl Executor {
    fn execute(&self) -> Result<(), String> {
        let exe_path = env::current_exe().map_err(|e| format!("Current exe error: {}", e))?;
        let parent_dir = exe_path.parent().ok_or_else(|| "Failed to determine exe directory".to_string())?;
        let safe_path: PathBuf = [parent_dir, Path::new("trusted"), Path::new(&self.command)].iter().collect();
        let safe_str = safe_path.to_str().ok_or_else(|| "Invalid path string".to_string())?;
        let mut child = Command::new(safe_str)
            .spawn()
            .map_err(|e| format!("Spawn error: {}", e))?;
        let status = child.wait().map_err(|e| format!("Wait error: {}", e))?;
        if status.success() {
            Ok(())
        } else {
            Err("Process did not exit successfully".to_string())
        }
    }
}

fn main() {
    let exec = Arc::new(Mutex::new(Executor {
        command: "helper".to_string(),
    }));
    let mut handles = vec![];
    for _ in 0..3 {
        let exec_clone = Arc::clone(&exec);
        let handle = thread::spawn(move || {
            let instance = exec_clone.lock().unwrap();
            instance.execute().unwrap();
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

/////////////////////// Test Oracle ///////////////////////
// The following test oracle is designed to differentiate between the vulnerable and fixed implementations.
// It creates two executables:
// 1. A "malicious" helper in the current working directory that always fails (exit code 1).
// 2. A "trusted" helper in a secure "trusted" subdirectory (located alongside the current executable) that succeeds (exit code 0).
//
// When running the vulnerable version, the process will find and run the malicious helper (due to the untrusted search path)
// and thus fail (resulting in an error). In contrast, the fixed version will construct an absolute path to the trusted helper,
// and its execution will succeed. The test asserts that execute() returns Ok(), and will fail if the vulnerability is present.

#[cfg(test)]
mod tests {
    use super::Executor;
    use std::env;
    use std::fs::{self, File, remove_file, create_dir_all};
    use std::io::Write;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_executable_search_path() {
        // Determine the current working directory.
        let cwd = env::current_dir().expect("Failed to get current directory.");
        let helper_name = "helper";

        // 1. Create a malicious helper in the current working directory.
        let cwd_helper_path = cwd.join(helper_name);
        {
            let mut file = File::create(&cwd_helper_path)
                .expect("Failed to create malicious helper in current directory.");
            // On Unix, create a shell script that exits with error.
            if cfg!(unix) {
                writeln!(file, "#!/bin/sh").unwrap();
                writeln!(file, "exit 1").unwrap();
            } else {
                writeln!(file, "exit 1").unwrap();
            }
        }
        // Set executable permission on Unix.
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&cwd_helper_path).unwrap().permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&cwd_helper_path, perms).unwrap();
        }

        // 2. Create a trusted helper in the secure "trusted" subdirectory.
        let exe_path = env::current_exe().expect("Failed to determine current executable path.");
        let exe_dir = exe_path.parent().expect("Failed to get executable directory.");
        let trusted_dir = exe_dir.join("trusted");
        create_dir_all(&trusted_dir).expect("Failed to create trusted directory.");
        let trusted_helper_path = trusted_dir.join(helper_name);
        {
            let mut file = File::create(&trusted_helper_path)
                .expect("Failed to create trusted helper.");
            // The trusted helper is a script that exits successfully.
            if cfg!(unix) {
                writeln!(file, "#!/bin/sh").unwrap();
                writeln!(file, "exit 0").unwrap();
            } else {
                writeln!(file, "exit 0").unwrap();
            }
        }
        // Set executable permission on Unix.
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&trusted_helper_path).unwrap().permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&trusted_helper_path, perms).unwrap();
        }

        // Instantiate the Executor with the helper command.
        let exec = Executor {
            command: helper_name.to_string(),
        };

        // Execute the command.
        let result = exec.execute();

        // Cleanup: remove the malicious helper and the trusted helper.
        let _ = remove_file(&cwd_helper_path);
        let _ = remove_file(&trusted_helper_path);

        // The expected behavior is:
        // - If the vulnerability exists (vulnerable version), the relative "helper" is found in the cwd (malicious) => result is Err.
        // - If fixed, the absolute path points to the trusted helper => result is Ok.
        assert!(result.is_ok(), "Test failed: Untrusted search path vulnerability triggered.");
    }
}
