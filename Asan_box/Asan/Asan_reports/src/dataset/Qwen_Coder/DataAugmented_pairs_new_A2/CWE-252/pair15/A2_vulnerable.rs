use std::sync::{Arc, Mutex};
use std::thread;

struct DataStore {
    buffer: [u8; 1024],
}

impl DataStore {
    unsafe fn load_data(&mut self, source: &[u8]) -> Result<(), &'static str> {
        if source.len() > self.buffer.len() {
            return Err("data exceeds capacity");
        }
        std::ptr::copy_nonoverlapping(source.as_ptr(), self.buffer.as_mut_ptr(), source.len());
        Ok(())
    }
}

fn perform_operation(input: &[u8]) -> Result<(), &'static str> {
    let shared_store = Arc::new(Mutex::new(DataStore { buffer: [0; 1024] }));
    let shared_clone = Arc::clone(&shared_store);
    let task_handle = thread::spawn(move || {
        let mut lock = shared_clone.lock().unwrap();
        unsafe {
            let _ = lock.load_data(input);
        }
    });
    let _ = task_handle.join();
    Ok(())
}

pub fn run_processing(input: &[u8]) -> Result<(), &'static str> {
    perform_operation(input)
}

fn main() {
    let input = vec![1u8; 2048];
    let _ = run_processing(&input);
    println!("Processing complete (vulnerable).");
}

--------------------------------------------------
#[cfg(test)]
mod tests {
    use super::run_processing;

    // This oracle test function calls run_processing with an oversized input.
    // For the vulnerable implementation, the unchecked error will cause the function
    // to erroneously return Ok(()), and the assertion will fail.
    // For the fixed implementation, the function correctly returns an Err.
    #[test]
    fn test_error_propagation() {
        let bad_input = vec![1u8; 2048];
        let result = run_processing(&bad_input);
        // We expect an error for oversized input.
        assert!(result.is_err(), "The processing function should fail on oversized input.");
    }
}
--------------------------------------------------
