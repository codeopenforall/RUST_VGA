#![allow(unused)]
use std::sync::Arc;
use std::thread;

struct DataProcessor {
    values: Vec<u32>,
}

impl DataProcessor {
    fn create(values: Vec<u32>) -> Self {
        DataProcessor { values }
    }

    fn fetch(&self, position: usize) -> Result<u32, &'static str> {
        unsafe {
            Ok(*self.values.as_ptr().add(position))
        }
    }
}

fn execute(index: usize) -> Result<u32, &'static str> {
    let processor = DataProcessor::create(vec![10, 20, 30, 40]);
    let processor_arc = Arc::new(processor);
    let processor_clone = Arc::clone(&processor_arc);
    let thread_handle = thread::spawn(move || {
        processor_clone.fetch(index)
    });
    thread_handle.join().unwrap()
}

fn main() {
    let idx = 4;
    match execute(idx) {
        Ok(value) => println!("Result: {}", value),
        Err(error) => println!("Error: {}", error),
    }
}