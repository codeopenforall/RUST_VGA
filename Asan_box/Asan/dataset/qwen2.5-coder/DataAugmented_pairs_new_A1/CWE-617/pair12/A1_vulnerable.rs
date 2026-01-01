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