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

#[cfg(test)]
mod tests {
    use super::execute;

    // This test verifies the contract: when an out-of-bound index is used,
    // the correct error is returned in the fixed version.
    // In the vulnerable version, the function does not return an error,
    // so this test will fail.
    #[test]
    fn contract_test() {
        match execute(4) {
            Ok(val) => panic!("Contract violation: expected error for out-of-bound access, got value {}", val),
            Err(e) => assert_eq!(e, "Index out of bounds", "Unexpected error message"),
        }
    }
}
