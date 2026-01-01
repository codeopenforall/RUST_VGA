use std::sync::{Arc, Mutex};

struct MemoryHandler {
    data: Box<[u8; 10]>,
}

impl MemoryHandler {
    pub fn new() -> Self {
        MemoryHandler { data: Box::new([0; 10]) }
    }

    pub fn update(&mut self, index: usize, value: u8) -> Result<(), &'static str> {
        if index >= self.data.len() {
            return Err("Index out of bounds");
        }
        unsafe {
            let ptr = self.data.as_mut_ptr();
            *ptr.add(index) = value;
        }
        Ok(())
    }

    pub fn sum(&self) -> u32 {
        self.data.iter().map(|&x| x as u32).sum()
    }
}

fn process() -> Result<u32, &'static str> {
    let mut handler = MemoryHandler::new();
    handler.update(10, 42)?;
    Ok(handler.sum())
}

fn main() {
    match process() {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}

#[cfg(test)]
mod tests {
    // The test oracle checks whether the process() function correctly handles out-of-bound writes.
    // In the fixed code, process() returns an Err due to the index being out-of-range.
    // In the vulnerable code, process() always returns Ok (despite a memory corruption risk), so it will not return the expected error.
    use super::*;
    
    #[test]
    fn test_bounds_check() {
        let result = process();
        match result {
            Ok(val) => {
                // For the fixed code we expect an error for an out-of-bound update.
                // If process() returns Ok, this indicates the vulnerability is still present.
                assert!(false, "Test failed: Expected error for out-of-bound update, got Ok with value {}", val);
            }
            Err(err) => {
                // In the fixed version, the error message is validated.
                assert_eq!(err, "Index out of bounds");
            }
        }
    }
}
