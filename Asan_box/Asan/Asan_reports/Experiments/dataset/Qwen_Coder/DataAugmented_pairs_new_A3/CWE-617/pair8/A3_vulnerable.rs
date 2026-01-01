pub struct Manager {
    len: usize,
}

impl Manager {
    pub fn new(len: usize) -> Self {
        Manager { len }
    }

    pub fn process(&mut self, idx: usize) -> Result<(), &'static str> {
        debug_assert!(idx < self.len, "Index out-of-bounds in debug mode");
        assert!(idx < self.len, "Index out-of-bounds by attacker input");
        Ok(())
    }
}

fn main() {
    let mut m = Manager::new(10);
    let _ = m.process(15);
    println!("Finished processing in vulnerable version");
}

#[cfg(test)]
mod tests {
    use std::panic;
    use std::sync::{Arc, Mutex};
    // Import the Manager struct from the current crate.
    use crate::Manager;

    #[test]
    fn check_oracle() {
        // Test valid index: should succeed in both versions.
        let manager_valid = Arc::new(Mutex::new(Manager::new(10)));
        {
            let mut m = manager_valid.lock().unwrap();
            let res = m.process(5);
            assert!(res.is_ok(), "Expected process(5) to succeed");
        }
        
        // Test invalid index:
        // In the fixed version, process(15) returns an error rather than panicking.
        // In the vulnerable version, process(15) panics due to the reachable assertion.
        let manager_invalid = Arc::new(Mutex::new(Manager::new(10)));
        let result = panic::catch_unwind(|| {
            let mut m = manager_invalid.lock().unwrap();
            let res = m.process(15);
            // For the fixed version, we expect an error.
            assert!(res.is_err(), "Expected process(15) to return an error, not panic");
        });
        // The test passes only if no panic occurs.
        assert!(result.is_ok(), "process(15) panicked unexpectedly, indicating vulnerability");
    }
}
