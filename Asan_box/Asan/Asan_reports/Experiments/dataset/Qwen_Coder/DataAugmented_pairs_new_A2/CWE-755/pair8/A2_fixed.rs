use std::sync::{Arc, Mutex};
use std::thread;

struct DataProcessor {
    data: Arc<Mutex<Vec<u32>>>,
}

impl DataProcessor {
    fn compute(&self) -> Result<u32, &'static str> {
        let data = self.data.lock().map_err(|_| "Lock poisoned")?;
        if data.is_empty() {
            return Err("Empty dataset");
        }
        let sum: u32 = data.iter().sum();
        let count = data.len() as u32;
        sum.checked_div(count).ok_or("Division error")
    }
}

trait Average {
    fn average(&self) -> Result<u32, &'static str>;
}

impl Average for DataProcessor {
    fn average(&self) -> Result<u32, &'static str> {
        self.compute()
    }
}

fn main() {
    let processor = DataProcessor { data: Arc::new(Mutex::new(vec![0; 5])) };
    {
        let mut data = processor.data.lock().unwrap();
        unsafe {
            let ptr = data.as_mut_ptr();
            for i in 0..data.len() {
                *ptr.add(i) = (i as u32) + 1;
            }
        }
    }
    match processor.average() {
        Ok(val) => println!("Output: {}", val),
        Err(e) => println!("Error: {}", e),
    }
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
