use std::sync::{Arc, Mutex};
use std::thread;

struct LogHolder {
    secret: &'static str,
}

impl LogHolder {
    fn new() -> Self {
        LogHolder { secret: "VerySecretInfo" }
    }

    fn process(&self, user_fmt: &str) -> String {
        let secret_ptr = self.secret.as_ptr();
        let secret_str: &str = unsafe {
            let slice = std::slice::from_raw_parts(secret_ptr, self.secret.len());
            std::str::from_utf8_unchecked(slice)
        };
        format!(user_fmt, secret_str)
    }
}

fn simulate(user_input: &str) -> String {
    let logger = Arc::new(LogHolder::new());
    let output = Arc::new(Mutex::new(String::new()));
    let logger_clone = Arc::clone(&logger);
    let output_clone = Arc::clone(&output);
    let handle = thread::spawn(move || {
        let result = logger_clone.process(user_input);
        let mut out_lock = output_clone.lock().unwrap();
        *out_lock = result;
    });
    handle.join().unwrap();
    let res = output.lock().unwrap();
    res.clone()
}

fn main() {
    let user_controlled_input = "Alert: {}";
    let result = simulate(user_controlled_input);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    // This test oracle validates the behavior under malicious input.
    // It expects that the returned string DOES NOT contain the sensitive secret.
    // In the vulnerable version, using malicious input like "Alert: {}" would include the secret,
    // causing this test to fail. The fixed version avoids revealing the secret.
    #[test]
    fn test_format_string_handling() {
        let malicious_input = "Alert: {}";
        let secret = "VerySecretInfo";

        // Call the simulate function from the code under test.
        let output = simulate(malicious_input);

        // The oracle asserts that the secret is not present in the output.
        // For a secure implementation, the secret should not have been formatted into the output.
        assert!(
            !output.contains(secret),
            "Sensitive information was leaked in the output!"
        );
    }
}
