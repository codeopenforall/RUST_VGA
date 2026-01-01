use std::sync::{Arc, Mutex};
use std::thread;

struct Secure {
    secret: String,
    logs: Arc<Mutex<Vec<String>>>,
}

impl Secure {
    fn new(secret: String) -> Secure {
        Secure { 
            secret, 
            logs: Arc::new(Mutex::new(Vec::new()))
        }
    }

    fn run_task(&self, index: usize) {
        if index > self.secret.len() {
            unsafe {
                let _byte = *self.secret.as_ptr().offset(index as isize);
                panic!("Access error: secret leak: {}. Index: {}", self.secret, index);
            }
        } else {
            let msg = format!("Access OK at index {}.", index);
            let mut logs = self.logs.lock().unwrap();
            logs.push(msg);
        }
    }
}

fn main() {
    let secure = Arc::new(Secure::new("SuperSecretPassword".to_string()));
    let handles: Vec<_> = (0..3).map(|i| {
        let sec = secure.clone();
        thread::spawn(move || {
            sec.run_task(i);
        })
    }).collect();
    for handle in handles {
        let _ = handle.join();
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::panic;
    // For testing, expect the Secure struct from the current module.
    // When compiled with vulnerable code, the panic message will include the secret.
    // With the fixed code, the secret will not be present.
    // NOTE: Replace `crate::Secure` with the appropriate module path if necessary.
    use crate::Secure;

    // Helper function to capture panic messages.
    fn capture_panic<F: FnOnce() + std::panic::UnwindSafe>(f: F) -> String {
        match panic::catch_unwind(f) {
            Ok(_) => "No panic".to_string(),
            Err(err) => {
                if let Some(s) = err.downcast_ref::<&str>() {
                    s.to_string()
                } else if let Some(s) = err.downcast_ref::<String>() {
                    s.to_string()
                } else {
                    "Panic with non-string payload".to_string()
                }
            }
        }
    }

    #[test]
    fn test_sensitive_not_leaked() {
        let secure = Arc::new(Secure::new("SuperSecretPassword".to_string()));
        // Use an index that forces the error condition.
        let res = capture_panic(|| {
            secure.run_task(100);
        });
        // In the vulnerable version, res will include "SuperSecretPassword".
        // In the fixed version, res should not include it.
        assert!(
            !res.contains("SuperSecretPassword"),
            "Test failed: Sensitive data was exposed in error message."
        );
    }
}
