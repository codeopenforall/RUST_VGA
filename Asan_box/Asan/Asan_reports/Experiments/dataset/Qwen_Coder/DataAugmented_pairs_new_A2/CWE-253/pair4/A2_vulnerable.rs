use std::sync::{Arc, Mutex};
use std::thread;

struct Processor {
    storage: Vec<u8>,
}

impl Processor {
    fn new() -> Self {
        Processor { storage: vec![0; 64] }
    }

    unsafe fn execute(&mut self, data: &[u8]) -> Result<(), &'static str> {
        if data.len() > self.storage.len() {
            return Err("Data size exceeds storage capacity");
        }
        std::ptr::copy_nonoverlapping(data.as_ptr(), self.storage.as_mut_ptr(), data.len());
        Ok(())
    }

    fn process(&mut self, data: &[u8]) -> Result<(), &'static str> {
        let outcome = unsafe { self.execute(data) };
        if outcome.is_err() {
            return Ok(());  
        }
        outcome
    }
}

fn main() {
    let processor = Arc::new(Mutex::new(Processor::new()));
    let input_data = vec![1u8; 100]; 
    let thread_task = {
        let shared_processor = Arc::clone(&processor);
        thread::spawn(move || {
            let mut handle = shared_processor.lock().unwrap();
            let _ = handle.process(&input_data);
        })
    };
    thread_task.join().unwrap();
    println!("Process completed");
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
