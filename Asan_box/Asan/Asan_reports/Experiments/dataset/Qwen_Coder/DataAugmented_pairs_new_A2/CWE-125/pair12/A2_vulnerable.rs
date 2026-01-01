use std::sync::Arc;
use std::thread;

struct DataHandler {
    data: Vec<u32>,
}

impl DataHandler {
    fn new(vec: Vec<u32>) -> Self {
        Self { data: vec }
    }

    fn get_item(&self, index: usize) -> Result<u32, &'static str> {
        unsafe {
            let ptr = self.data.as_ptr();
            if index >= self.data.len() {
                return Err("Index out-of-range");
            }
            Ok(*ptr.add(index))
        }
    }
}

fn main() {
    let handler = Arc::new(DataHandler::new(vec![100, 200, 300, 400]));
    let handler_clone = Arc::clone(&handler);
    let handle = thread::spawn(move || {
        match handler_clone.get_item(4) {
            Ok(val) => println!("Read value: {}", val),
            Err(e) => println!("Error: {}", e),
        }
    });
    handle.join().unwrap();
}

/*
   The following test oracle function serves as the contract for this pair of implementations.
   It calls the get_item method with an index equal to the length of the buffer.
   - In the secure/fixed version, the function returns an Err("Index out-of-range"), and the test passes.
   - In the insecure/vulnerable version, an out-of-bounds read occurs (or it returns a spurious value), 
     and the test will fail the assertion expecting an error.
     
   This single test demonstrates that the secure version properly handles the boundary condition.
*/
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_oracle() {
        let handler = DataHandler::new(vec![100, 200, 300, 400]);
        let result = handler.get_item(4);
        assert!(
            result.is_err(),
            "Expected error for out-of-bound access, but got: {:?}",
            result
        );
    }
}
