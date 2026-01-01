use std::sync::{Arc, Mutex};
use std::thread;

struct Storage {
    data: Vec<u32>,
}

impl Storage {
    fn create() -> Self {
        Storage { data: vec![0; 10] }
    }
    fn modify(&mut self, idx: usize, val: u32) {
        unsafe {
            let ptr = self.data.as_mut_ptr();
            *ptr.add(idx) = val;
        }
        assert!(self.data[idx] < 1000, "Constraint breached: value must be below 1000");
    }
}

fn handle_input(value: u32) {
    let shared_storage = Arc::new(Mutex::new(Storage::create()));
    let mut thread_handles = vec![];

    for i in 0..3 {
        let storage_clone = Arc::clone(&shared_storage);
        let val = value;
        let thread = thread::spawn(move || {
            let mut lock = storage_clone.lock().unwrap();
            if i == 1 {
                lock.modify(2, val);
            } else {
                lock.modify(i, 42);
            }
        });
        thread_handles.push(thread);
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }
}

fn main() {
    handle_input(1500);
}

/*
This test oracle function is designed to confirm the exploitability of the vulnerability in the vulnerable version 
and its remediation in the fixed version. The test calls the process() function with a deliberately invalid input (1500)
that should trigger a panic in the vulnerable variant, while the fixed variant should complete without panicking.

When linked with the vulnerable code, the test should fail because a panic occurs. Conversely, when linked with the fixed 
version, the test should pass.
*/
#[cfg(test)]
mod tests {
    use std::panic;

    // Import the process function.
    use super::process;

    #[test]
    fn oracle_test() {
        let result = panic::catch_unwind(|| {
            process(1500);
        });
        // In a secure implementation, the processing with invalid input should not panic.
        assert!(result.is_ok(), "Test oracle failed: operation panicked due to attacker-controlled input.");
    }
}
