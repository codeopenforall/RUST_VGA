use std::sync::{Arc, Mutex};
use std::thread;

struct Manager {
    buffer: Vec<u32>,
    marker: u32,
}

impl Manager {
    fn new(size: usize) -> Self {
        Manager {
            buffer: vec![0; size],
            marker: 0,
        }
    }

    fn process(&mut self, idx: usize) -> Result<(), &'static str> {
        let len = self.buffer.len();
        if idx >= len {
            return Err("Index out-of-bounds");
        }
        unsafe {
            let ptr = self.buffer.as_mut_ptr();
            *ptr.add(idx) = 42;
        }
        self.marker = 1;
        Ok(())
    }
}

fn main() {
    let manager = Arc::new(Mutex::new(Manager::new(10)));
    let mgr_clone = Arc::clone(&manager);
    let t = thread::spawn(move || {
        let mut m = mgr_clone.lock().unwrap();
        assert!(m.process(5).is_ok(), "Process should succeed for valid index");
    });
    let _ = t.join().unwrap();
    println!("Finished processing in fixed version");
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
