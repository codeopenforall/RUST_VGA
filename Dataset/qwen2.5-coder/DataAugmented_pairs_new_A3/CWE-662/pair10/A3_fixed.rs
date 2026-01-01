use std::io::{Error, ErrorKind};
use std::sync::{Arc, Mutex};

struct DataProcessor {
    data: Arc<Mutex<Vec<u8>>>,
}

impl DataProcessor {
    pub fn process(&self, input: &[u8]) -> Result<(), Error> {
        let vec_guard = self.data.lock().unwrap();
        if input.len() > vec_guard.len() {
            return Err(Error::new(ErrorKind::Other, "Buffer overflow"));
        }
        unsafe { self.add_data(input) };
        Ok(())
    }

    unsafe fn add_data(&self, input: &[u8]) {
        // Simulate adding data to the buffer
        let mut vec_guard = self.data.lock().unwrap();
        vec_guard.clear();
        vec_guard.extend_from_slice(input);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn oracle() {
        // Set up a processor with a fixed-size buffer.
        let data = Arc::new(Mutex::new(vec![0u8; 10]));
        let processor = DataProcessor { data: data.clone() };

        // Valid input: should succeed.
        let res_valid = processor.process(&[1, 2, 3]);
        assert!(res_valid.is_ok(), "Valid input should succeed.");

        // Invalid input: exceeds the buffer size.
        let res_invalid = processor.process(&[1,2,3,4,5,6,7,8,9,10,11]);
        // The oracle expects an error for invalid input.
        assert!(res_invalid.is_err(), "Invalid input should produce an error.");
    }
}