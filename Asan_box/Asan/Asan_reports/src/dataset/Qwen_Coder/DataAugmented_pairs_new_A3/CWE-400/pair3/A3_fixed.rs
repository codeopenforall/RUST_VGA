use std::sync::{Arc, Mutex};

const MAX_CAPACITY: usize = 1024;

pub struct Processor {
    data: Vec<u8>,
}

impl Processor {
    pub fn append_checked(&mut self, item: u8) -> Result<(), &'static str> {
        if self.data.len() >= MAX_CAPACITY {
            return Err("Resource limit reached");
        }
        self.data.push(item);
        Ok(())
    }
}

pub fn simulate_input(input: &[u8], proc: &Arc<Mutex<Processor>>) -> Result<(), &'static str> {
    for &byte in input {
        let mut locked = proc.lock().unwrap();
        locked.append_checked(byte)?;
    }
    Ok(())
}

//////////////////////////
// Test Oracle Function
//////////////////////////
#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    // Import the public types and the simulate_input function from the respective module.
    // Note: Adjust the import path if necessary.
    use super::{simulate_input, Processor};

    #[test]
    fn test_resource_enforcement() {
        // Setup a Processor instance.
        let proc = Arc::new(Mutex::new(Processor { data: Vec::new() }));
        // Create an input that exceeds the safe capacity limit.
        let excessive_input = vec![42u8; 1500]; // 1500 bytes; MAX_CAPACITY is 1024 in the fixed version.
        
        let outcome = simulate_input(&excessive_input, &proc);
        
        // For the fixed version, we expect an error once the limit is reached.
        // The vulnerable version would erroneously process all data and return Ok.
        assert!(outcome.is_err(), "Resource limit not enforced as expected");
    }
}
