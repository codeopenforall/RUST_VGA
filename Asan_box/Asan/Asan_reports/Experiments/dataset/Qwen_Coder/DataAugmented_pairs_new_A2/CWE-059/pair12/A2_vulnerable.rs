use std::sync::Arc;
use std::thread;

struct Manager {
    base: String,
}

impl Manager {
    fn new(base: &str) -> Self {
        Manager { base: base.to_string() }
    }

    fn open(&self, input: &str) -> Result<String, &'static str> {
        let full_path = unsafe {
            let base_ptr = self.base.as_ptr();
            let input_ptr = input.as_ptr();
            let base_len = self.base.len();
            let input_len = input.len();
            let mut vec = Vec::with_capacity(base_len + input_len);
            vec.extend_from_slice(self.base.as_bytes());
            vec.extend_from_slice(input.as_bytes());
            String::from_utf8_unchecked(vec)
        };
        if full_path.contains("..") {
            Ok("sensitive_data".to_owned())
        } else {
            Ok("regular_data".to_owned())
        }
    }
}

fn main() {
    let manager = Arc::new(Manager::new("/safe_dir/"));
    let manager_clone = Arc::clone(&manager);

    let handle = thread::spawn(move || {
        let result = manager_clone.open("../etc/passwd");
        println!("Thread result: {}", result.unwrap());
    });

    handle.join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    // This test oracle should be used separately against both versions.
    // For the vulnerable implementation, the test should fail because the malicious input returns "sensitive_data".
    // For the fixed implementation, the test should pass because the malicious input is rejected.
    #[test]
    fn test_file_access() {
        let manager = Arc::new(Manager::new("/safe_dir/"));
        // Simulated traversal input.
        let malicious = manager.open("../etc/passwd");
        // In the secure (fixed) version, traversal is denied.
        assert!(malicious.is_err(), "Traversal input should be denied.");

        // Test safe access.
        let safe = manager.open("file.txt");
        assert!(safe.is_ok(), "Safe access should succeed.");
        assert_eq!(safe.unwrap(), "regular_data");
    }
}
