use std::sync::{Arc, Mutex};
use std::thread;

struct Processor {
    count: usize,
    max_count: usize,
}

impl Processor {
    fn initialize(max: usize) -> Self {
        Processor { count: 0, max_count: max }
    }

    fn adjust(&mut self, value: usize) {
        unsafe {
            let ptr = &mut self.count as *mut usize;
            *ptr = value;
        }
        assert!(self.count <= self.max_count, "Count exceeds maximum allowed");
    }
}

pub fn process_input(input: usize) -> Result<(), String> {
    let mut processor = Processor::initialize(100);
    processor.adjust(input);
    Ok(())
}

fn main() {
    let processor = Arc::new(Mutex::new(Processor::initialize(100)));
    let processor_clone = Arc::clone(&processor);
    let handle = thread::spawn(move || {
        let mut processor = processor_clone.lock().unwrap();
        processor.adjust(150);
    });
    let _ = handle.join();
    println!("Completed processing in vulnerable version.");
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Test Oracle: This unit test verifies the behavior of the public process_input API.
// It uses catch_unwind to safely capture panics. For the vulnerable version, the assertion will
// trigger and cause a panic (test failure). For the fixed version, process_input returns an Err,
// indicating that the input was safely rejected, and no panic occurs.
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn test_process_input() {
        let result = panic::catch_unwind(|| process_input(150));
        match result {
            Ok(res) => {
                // In the fixed version, process_input does not panic and returns Err.
                assert!(res.is_err(), "Expected an error result for excessive input in safe version");
            },
            Err(_) => {
                panic!("process_input panicked, indicating a vulnerability");
            }
        }
    }
}
