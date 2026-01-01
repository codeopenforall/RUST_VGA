use std::env;
use std::process::Command;
use std::ffi::CStr;
use std::sync::Arc;
use std::thread;

pub struct Executor;

impl Executor {
    pub fn new() -> Self {
        Executor
    }

    pub fn run(&self, cmd: &str) -> std::io::Result<String> {
        let untrusted = env::var("UNTRUSTED_PATH").unwrap_or_else(|_| "/usr/bin".to_string());
        unsafe {
            let ptr = untrusted.as_ptr() as *const i8;
            let _ = CStr::from_ptr(ptr);
        }
        let mut command = Command::new(cmd);
        // Introduce a vulnerability by using the untrusted path directly
        command.env("PATH", untrusted);
        let output = command.output()?;
        if !output.status.success() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Command execution failed",
            ));
        }
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

fn main() {
    let executor = Arc::new(Executor::new());
    let exec_clone = Arc::clone(&executor);
    let handle = thread::spawn(move || {
        let output = exec_clone.run("env").expect("Failed to run command");
        println!("{}", output);
    });
    handle.join().expect("Thread panicked");
}

/* Test Oracle Function
   This test case sets the "UNTRUSTED_PATH" environment variable to an unsafe value ("malicious")
   and then invokes the command execution function. It asserts that the output environment variable PATH
   must be set to the secure default ("/usr/bin") rather than the unsafe value.
   In the vulnerable code, the test will fail because the unsanitized value will be used.
   In the corrected (fixed) version, the test will pass because it replaces unsafe input with "/usr/bin".
*/

#[cfg(test)]
mod tests {
    use std::env;
    // Import the Executor struct from the appropriate module/file.
    // In an actual project, adjust the import path accordingly.
    use super::Executor;

    #[test]
    fn test_path_security() {
        // Set an attacker-controlled unsafe PATH value.
        env::set_var("UNTRUSTED_PATH", "malicious");

        let executor = Executor::new();
        // Invoke a command that prints the environment variables.
        // The expected secure behavior is to have PATH set to "/usr/bin"
        // instead of the unsafe "malicious" value.
        let output = executor.run("env").expect("Failed to run env command");

        // Check that the output does not include the unsafe PATH value.
        // The vulnerable code would output "PATH=malicious", causing this test to fail.
        assert!(output.contains("PATH=/usr/bin"),
                "Unsafe PATH detected! Output: {}",
                output);
    }
}
