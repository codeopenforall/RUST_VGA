use std::sync::{Arc, Mutex};
use std::thread;

struct Resource {
    data: Mutex<Vec<u8>>,
}

impl Resource {
    fn new() -> Self {
        Resource { data: Mutex::new(Vec::new()) }
    }

    unsafe fn write_data(&self, bytes: &[u8]) -> Result<(), &'static str> {
        if bytes.is_empty() {
            return Err("No data to write");
        }
        let mut lock = self.data.lock().unwrap();
        lock.extend_from_slice(bytes);
        Ok(())
    }
}

fn run_update(resource: &Arc<Resource>, input: &[u8]) {
    let res_clone = Arc::clone(resource);
    let handle = thread::spawn(move || {
        unsafe {
            let _ = res_clone.write_data(input);
        }
    });
    let _ = handle.join();
}

pub fn process_update(resource: &Arc<Resource>, input: &[u8]) -> bool {
    if input.is_empty() {
        return false;
    }
    run_update(resource, input);
    true
}

fn main() {
    let res = Arc::new(Resource::new());
    let outcome = process_update(&res, b"");
    println!("Outcome: {}", outcome);
    println!("Data: {:?}", res.data.lock().unwrap());
}

//////////////////////
// Test Oracle Code //
//////////////////////

// This test should be placed in a separate test file or module.
// It verifies that when an empty slice is provided, the update process
// fails (i.e., process_update returns false).
// For the corrected version, the test passes.
// For the vulnerable version, the test fails since process_update erroneously returns true.

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_process_update_empty_input() {
        let res = Arc::new(Resource::new());
        // Passing an empty slice should cause the update to fail.
        let result = process_update(&res, b"");
        assert_eq!(result, false, "Expected update failure for empty input.");
    }
}
