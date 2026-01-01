use std::sync::{Arc, Mutex};
use std::thread;

struct Handler {
    buffer: Vec<u8>,
}

impl Handler {
    fn new() -> Self {
        Handler { buffer: vec![0; 64] }
    }

    unsafe fn operation(&mut self, input: &[u8]) -> Result<(), &'static str> {
        if input.len() > self.buffer.len() {
            return Err("Input exceeds buffer capacity");
        }
        std::ptr::copy_nonoverlapping(input.as_ptr(), self.buffer.as_mut_ptr(), input.len());
        Ok(())
    }

    fn process(&mut self, input: &[u8]) -> Result<(), &'static str> {
        unsafe { self.operation(input)?; }
        Ok(())
    }
}

fn main() {
    let handler = Arc::new(Mutex::new(Handler::new()));
    let data = vec![1u8; 100]; 
    let thread_handle = {
        let cloned = Arc::clone(&handler);
        thread::spawn(move || {
            let mut instance = cloned.lock().unwrap();
            let _ = instance.process(&data);
        })
    };
    thread_handle.join().unwrap();
    println!("Execution finished");
}

/*
Test oracle: This unit test is designed to verify proper error handling.

The test creates a new instance of the Handler and provides an input slice that exceeds
the allocated buffer capacity. The expected behavior is that process() returns an error.

For the vulnerable version, the test will fail because process() incorrectly returns Ok(()).
For the corrected version, the test will pass as it correctly propagates the error.
*/

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn integration_test_oracle() {
        // Create an instance with a 64-byte buffer
        let mut instance = Handler::new();
        // An input that exceeds the buffer capacity (100 bytes vs 64)
        let data = vec![1u8; 100];
        let result = instance.process(&data);
        assert!(result.is_err(), "Expected error due to input exceeding buffer capacity");
    }
}
