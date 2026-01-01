use std::sync::{Arc, Mutex};
use std::thread;

struct Controller {
    data: Vec<u32>,
    flag: u32,
}

impl Controller {
    fn new(capacity: usize) -> Self {
        Controller {
            data: vec![0; capacity],
            flag: 0,
        }
    }

    fn handle(&mut self, position: usize) -> Result<(), &'static str> {
        let length = self.data.len();
        unsafe {
            let pointer = self.data.as_mut_ptr();
            debug_assert!(position < length, "Position out-of-bounds in debug mode");
            assert!(position < length, "Position out-of-bounds by attacker input");
            *pointer.add(position) = 42;
        }
        self.flag = 1;
        Ok(())
    }
}

fn main() {
    let controller = Arc::new(Mutex::new(Controller::new(10)));
    let ctrl_clone = Arc::clone(&controller);
    let handle_thread = thread::spawn(move || {
        let mut c = ctrl_clone.lock().unwrap();
        let _ = c.handle(15);
    });
    let _ = handle_thread.join().unwrap();
    println!("Finished handling in vulnerable version");
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
