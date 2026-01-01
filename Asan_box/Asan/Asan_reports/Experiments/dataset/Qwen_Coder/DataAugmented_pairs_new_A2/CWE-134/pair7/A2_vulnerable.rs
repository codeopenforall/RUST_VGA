use std::sync::{Arc, Mutex};
use std::thread;
use std::env;

trait Formatter {
    unsafe fn execute(&self, fmt: &str) -> String;
}

struct Kernel {
    tracker: Arc<Mutex<i32>>,
}

impl Kernel {
    fn initialize() -> Self {
        Self {
            tracker: Arc::new(Mutex::new(0)),
        }
    }

    fn modify(&self, value: i32) {
        let mut lock = self.tracker.lock().unwrap();
        *lock = value;
    }
}

impl Formatter for Kernel {
    unsafe fn execute(&self, fmt: &str) -> String {
        let count = *self.tracker.lock().unwrap();
        format!(fmt, count)
    }
}

fn main() {
    let kernel = Kernel::initialize();
    kernel.modify(123);
    let shared_kernel = Arc::new(kernel);
    let format_input = env::args().nth(1).unwrap_or_else(|| "Tracker: {}".to_string());
    let cloned_kernel = Arc::clone(&shared_kernel);
    let thread_handle = thread::spawn(move || {
        unsafe {
            let result = cloned_kernel.execute(&format_input);
            println!("{}", result);
        }
    });
    thread_handle.join().unwrap();
}

#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::env;

    // The following test oracle assumes that the vulnerable version compiles to a binary named "vulnerable_binary"
    // and the corrected version compiles to a binary named "fixed_binary". Adjust the binary names accordingly.

    // Test for the vulnerable variant: expects the process to fail (panic) when given a malicious format string.
    #[test]
    fn test_externally_controlled_format_insecure() {
        // This malicious input has extra placeholder leading to a panic.
        let malicious = "Exploit attempt: {} {}";
        let output = Command::new("cargo")
            .args(&["run", "--bin", "vulnerable_binary", "--", malicious])
            .output()
            .expect("Failed to execute vulnerable binary process");
            
        // The vulnerable version is expected to panic and thus exit with a non-zero status.
        assert!(!output.status.success(), "Insecure version did not fail as expected");
    }

    // Test for the corrected variant: the process should execute successfully.
    #[test]
    fn test_externally_controlled_format_secure() {
        // Even with a malicious string, the corrected version treats it as plain text.
        let malicious = "Exploit attempt: {} {}";
        let output = Command::new("cargo")
            .args(&["run", "--bin", "fixed_binary", "--", malicious])
            .output()
            .expect("Failed to execute fixed binary process");
            
        assert!(output.status.success(), "Secure version failed unexpectedly");
        let stdout = String::from_utf8_lossy(&output.stdout);
        // The output should include the entire malicious string and the counter value.
        assert!(stdout.contains(malicious), "Output does not contain the injected message");
        assert!(stdout.contains("123"), "Output does not contain the counter value");
    }
}
