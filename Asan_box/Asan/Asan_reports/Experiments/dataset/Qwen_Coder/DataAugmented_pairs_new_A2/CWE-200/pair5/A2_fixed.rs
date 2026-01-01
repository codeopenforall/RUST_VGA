use std::sync::{Arc, Mutex};
use std::thread;

struct SecureData {
    secret: String,
}

impl SecureData {
    fn new(secret: String) -> Self {
        SecureData { secret }
    }

    fn process(&self, value: i32) -> Result<i32, String> {
        if value < 0 || value > 100 {
            return Err(format!("Invalid value: {}. Operation aborted.", value));
        }
        Ok(value * 2)
    }
}

fn main() {
    let data = Arc::new(Mutex::new(SecureData::new("SuperSecret123".to_string())));
    let mut threads = vec![];
    for _ in 0..4 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let guard = data_clone.lock().unwrap();
            match guard.process(101) {
                Ok(result) => println!("Computation succeeded: {}", result),
                Err(err) => println!("Computation error: {}", err),
            }
        });
        threads.push(handle);
    }
    for handle in threads {
        let _ = handle.join();
    }
}

/////////////////// Test Oracle ///////////////////
// This test oracle function asserts that a call to process() with an invalid value
// does not expose the secret ("SuperSecret123") in its error message. For the vulnerable
// version, this test will fail due to the sensitive information leak, while for the fixed
// version, it will pass.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sensitive_data_not_exposed() {
        let instance = SecureData::new("SuperSecret123".to_string());
        let response = instance.process(101);
        match response {
            Err(err_msg) => {
                // The oracle checks that the error message does NOT contain the sensitive string.
                assert!(
                    !err_msg.contains("SuperSecret123"),
                    "Test failed: sensitive data exposure detected"
                );
            },
            Ok(_) => panic!("Test failed: invalid input should produce an error"),
        }
    }
}
