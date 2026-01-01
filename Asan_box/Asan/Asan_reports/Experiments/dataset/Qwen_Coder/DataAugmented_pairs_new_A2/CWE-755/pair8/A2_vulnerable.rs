use std::sync::{Arc, Mutex};
use std::thread;

struct DataHandler {
    values: Arc<Mutex<Vec<u32>>>,
}

impl DataHandler {
    fn calculate(&self) -> u32 {
        let data = self.values.lock().unwrap();
        if data.is_empty() {
            100u32 / 0 // Introducing division by zero defect
        } else {
            let total: u32 = data.iter().sum();
            let length = data.len() as u32;
            total / length
        }
    }
}

trait Value {
    fn value(&self) -> Result<u32, &'static str>;
}

impl Value for DataHandler {
    fn value(&self) -> Result<u32, &'static str> {
        Ok(self.calculate())
    }
}

fn main() {
    let handler = DataHandler { values: Arc::new(Mutex::new(vec![])) };
    let handler_clone = DataHandler { values: Arc::clone(&handler.values) };
    let thread_handle = thread::spawn(move || {
        unsafe {
            let data = handler_clone.values.lock().unwrap();
            let ptr = data.as_ptr();
            for i in 0..data.len() {
                let _ = *ptr.add(i);
            }
        }
    });
    thread_handle.join().unwrap();
    println!("Result: {:?}", handler.value());
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use super::{DataProcessor, Average};

    #[test]
    fn test_exception_handling() {
        // Create a DataProcessor with an empty vector to trigger exceptional conditions.
        let processor = DataProcessor { data: Arc::new(Mutex::new(vec![])) };

        // For a correct implementation, an error should be returned for an empty dataset.
        let result = processor.average();

        match result {
            // In the vulnerable implementation, a default value (0) is returned, which is incorrect.
            Ok(val) => panic!("Test failed: Expected error, but got Ok({})", val),
            Err(msg) => assert_eq!(msg, "Empty dataset", "Unexpected error message"),
        }
    }
}
