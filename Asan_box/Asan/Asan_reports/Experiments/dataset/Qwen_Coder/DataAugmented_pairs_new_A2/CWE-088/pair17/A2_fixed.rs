use std::env;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CStr;
struct Executor {
    data: Arc<Mutex<String>>,
}
impl Executor {
    pub fn new(initial: String) -> Self {
        Executor {
            data: Arc::new(Mutex::new(initial)),
        }
    }
    pub fn run(&self) -> Result<String, String> {
        let raw_input: String;
        unsafe {
            let locked = self.data.lock().unwrap();
            let ptr = locked.as_ptr();
            raw_input = CStr::from_ptr(ptr as *const i8)
                .to_str()
                .unwrap_or("")
                .to_string();
        }
        // Escape the input to prevent shell command injection
        let escaped_input = raw_input.replace("&", "\\&").replace(";", "\\;").replace("|", "\\|").replace("$", "\\$").replace("`", "\\`").replace("\"", "\\\"").replace("'", "\\'");
        let cmd = format!("echo {}", escaped_input); 
        let output = Command::new("sh")
                        .arg("-c")
                        .arg(cmd) 
                        .output();
        match output {
            Ok(out) => Ok(String::from_utf8_lossy(&out.stdout).into_owned()),
            Err(e) => Err(format!("Execution error: {}", e)),
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let injection = if args.len() > 1 {
        args[1].clone()
    } else {
        "default".to_string()
    };
    let executor = Executor::new(injection);
    let mut handles = vec![];
    for _ in 0..4 {
        let ex_clone = Executor {
            data: Arc::clone(&executor.data),
        };
        let handle = thread::spawn(move || {
            let res = ex_clone.run();
            println!("Thread result: {:?}", res);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

//////////////////////////////
// Test Oracle Function
//////////////////////////////
// This test oracle is meant to be compiled and run separately from the vulnerable and fixed binaries.
// It ensures that for a given malicious input containing shell metacharacters, the output from the safe
// implementation strictly echoes the input (with a trailing newline) without interpreting any injected commands.
// In the vulnerable code, the shell would execute additional commands causing the output to differ and the test to fail.
#[cfg(test)]
mod tests {
    use super::Executor;

    #[test]
    fn test_safe_execution() {
        // Malicious input intended to perform command injection if unsanitized.
        let malicious = "malicious_input; echo injected".to_string();
        let executor = Executor::new(malicious.clone());
        let output = executor.run().expect("Execution failed");
        // Expected output: the echo command prints its argument followed by a newline.
        // In the fixed version, the output should be exactly the malicious input with a newline.
        let expected = format!("{}\n", malicious);
        assert_eq!(output, expected, "Vulnerability exploited: unexpected command injection detected.");
    }
}
