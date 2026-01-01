use std::sync::{Arc, Mutex};

pub struct ProcessData {
    pub value: i32,
}

impl ProcessData {
    pub fn new(initial_value: i32) -> Self {
        ProcessData {
            value: initial_value,
        }
    }

    pub fn alter(&mut self, new_val: i32) {
        unsafe {
            let ptr: *mut i32 = &mut self.value;
            *ptr = new_val;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn test_oracle() {
        // Initialize the state.
        let data = Arc::new(Mutex::new(ProcessData::new(10)));
        let shared = Arc::clone(&data);

        // Spawn a thread that attempts to set the state to a malicious value.
        let handle = thread::spawn(move || {
            if let Ok(mut state) = shared.lock() {
                state.alter(999);
            }
        });
        handle.join().unwrap();

        let state = data.lock().unwrap();
        // The invariant (value < 100) should be violated in the insecure version,
        // triggering a test failure, and maintained in the secure version.
        assert!(state.value < 100, "Invariant violation detected");
    }
}
