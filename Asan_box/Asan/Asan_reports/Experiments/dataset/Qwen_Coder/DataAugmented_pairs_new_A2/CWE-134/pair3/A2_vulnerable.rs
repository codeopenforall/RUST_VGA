use std::env;
use std::ffi::CString;
use std::sync::{Arc, Mutex};
use std::thread;
extern "C" {
    fn printf(format: *const i8, ...) -> i32;
}
struct Logger {
    data: Mutex<String>,
}
impl Logger {
    fn new(s: &str) -> Self {
        Logger {
            data: Mutex::new(s.to_owned()),
        }
    }
    fn record(&self, input: &str) {
        let mut stored = self.data.lock().unwrap();
        *stored = input.to_owned();
        let fmt_str = CString::new(stored.as_str()).unwrap();
        unsafe {
            printf(fmt_str.as_ptr());
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let default = "No format provided";
    let user_in = if args.len() > 1 { &args[1] } else { default };
    let logger = Arc::new(Logger::new("Initial"));
    let logger_clone = Arc::clone(&logger);
    let handle = thread::spawn(move || {
        logger_clone.record(user_in);
    });
    handle.join().unwrap();
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    // This test assumes that the vulnerable and fixed binaries are built as "vulnerable_bin" and "fixed_bin"
    // respectively in the current directory.
    // The malicious input "%n" is expected to trigger undefined behavior in the vulnerable code (e.g. crash)
    // while the fixed version should handle it safely.
    #[test]
    fn check_format() {
        let malicious_input = "%n";

        // Testing the vulnerable binary: it should fail (exit with non-zero code).
        let vulnerable_output = Command::new("./vulnerable_bin")
            .arg(malicious_input)
            .output()
            .expect("failed to execute vulnerable binary");
        assert!(!vulnerable_output.status.success(), "The vulnerable binary should not succeed with malicious input");

        // Testing the fixed binary: it should succeed (exit with zero code).
        let fixed_output = Command::new("./fixed_bin")
            .arg(malicious_input)
            .output()
            .expect("failed to execute fixed binary");
        assert!(fixed_output.status.success(), "The fixed binary should succeed with malicious input");
    }
}
