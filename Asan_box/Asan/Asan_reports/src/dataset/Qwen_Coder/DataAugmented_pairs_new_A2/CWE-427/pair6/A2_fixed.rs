use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;

struct Executor;
impl Executor {
    fn run(&self, program: &str, args: &[&str]) -> std::io::Result<()> {
        let safe_path = "/usr/bin:/bin";
        let c_safe = CString::new(safe_path).expect("Failed to create CString");
        let mut cmd = Command::new(program);
        cmd.env("PATH", c_safe.to_str().unwrap())
           .args(args);
        let shared_cmd = Arc::new(Mutex::new(cmd));
        let thread_cmd = Arc::clone(&shared_cmd);
        let handle = thread::spawn(move || {
            let mut guard = thread_cmd.lock().unwrap();
            guard.spawn().and_then(|mut child| child.wait())
        });
        handle.join().unwrap()
    }
}

fn main() {
    let executor = Executor;
    executor.run("echo", &["Hello from safe execution!"]).unwrap();
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::process::Command;
    use std::path::PathBuf;

    // This oracle test launches the compiled binary with a malicious PATH.
    // The vulnerable version (when run under unsafe conditions) is expected to fail (non-zero exit code)
    // due to an attacker-controlled PATH that leads to a missing or rogue executable.
    // Conversely, the secured version sets its own safe PATH and should succeed.
    #[test]
    fn test_search_path() {
        // Determine the location of the current binary. It is assumed that the binary name is provided by Cargo.
        let exe_path = PathBuf::from(env!("CARGO_BIN_EXE_testtarget"));
        let output = Command::new(exe_path)
            // Set a malicious PATH to simulate an attack.
            .env("PATH", "/malicious/path")
            .output()
            .expect("Failed to execute the binary");

        // For the fixed implementation, this process should complete successfully.
        // For the vulnerable implementation, the manipulated PATH will likely cause the command to fail.
        assert!(output.status.success(), "Binary execution failed due to search path vulnerability.");
    }
}
