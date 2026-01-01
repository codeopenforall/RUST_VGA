use std::sync::{Arc, Mutex};
use std::thread;

struct Buffer {
    data: [u8; 1024],
}

impl Buffer {
    unsafe fn copy_into(&mut self, src: &[u8]) -> Result<(), &'static str> {
        if src.len() > self.data.len() {
            return Err("source too long");
        }
        std::ptr::copy_nonoverlapping(src.as_ptr(), self.data.as_mut_ptr(), src.len());
        Ok(())
    }
}

fn execute_task(input: &[u8]) -> Result<(), &'static str> {
    let shared = Arc::new(Mutex::new(Buffer { data: [0; 1024] }));
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        let mut guard = shared_clone.lock().unwrap();
        unsafe {
            guard.copy_into(input)
        }
    });
    match handle.join() {
        Ok(Ok(())) => Ok(()),
        Ok(Err(e)) => Err(e),
        Err(_) => Err("Thread panicked"),
    }
}

pub fn run_processing(input: &[u8]) -> Result<(), &'static str> {
    execute_task(input)
}

fn main() {
    let input = vec![1u8; 2048];
    let _ = run_processing(&input);
    println!("Operation completed (fixed).");
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
