use std::sync::{Arc, Mutex};
use std::io::{Result, Error, ErrorKind};
use std::thread;

struct DataProcessor {
    data: Arc<Mutex<Vec<u8>>>,
}

impl DataProcessor {
    unsafe fn add_data(&self, input: &[u8]) -> Result<()> {
        let mut vec_guard = self.data.lock().unwrap();
        if input.len() > vec_guard.len() {
            return Err(Error::new(ErrorKind::Other, "Buffer overflow"));
        }
        let raw_ptr = vec_guard.as_mut_ptr();
        for (i, &value) in input.iter().enumerate() {
            *raw_ptr.add(i) = value;
        }
        Ok(())
    }

    fn process(&self, input: &[u8]) -> Result<()> {
        unsafe {
            let _ = self.add_data(input); // Intentional defect: ignoring the result of add_data
        }
        Ok(())
    }
}

fn main() {
    let data = Arc::new(Mutex::new(vec![0u8; 10])); 
    let processor = DataProcessor { data: data.clone() };
    let clone_processor = DataProcessor { data: data.clone() };
    let thread_handle = thread::spawn(move || {
        let res = clone_processor.process(&[1,2,3,4,5,6,7,8,9,10,11]); 
        assert!(res.is_ok(), "Thread: Error should have been propagated, but it was ignored.");
    });
    let res_valid = processor.process(&[100,101,102]);
    assert!(res_valid.is_ok());
    thread_handle.join().unwrap();
    println!("Processing complete.");
}

/////////////////////////////////////////////////////////////
// Test Oracle Function for CWE-252 Unchecked Return Value
// This test function is intended to be used with both versions.
// It asserts that processing invalid input (i.e., input larger than the allocated
// buffer) returns an error. In the vulnerable version, the error is discarded so the
// test will fail, but in the corrected version, the error is correctly returned and
// the test will pass.
/////////////////////////////////////////////////////////////

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
