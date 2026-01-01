use std::sync::{Arc, Mutex};
use std::env;
use std::thread;
use std::time::Duration;

pub struct Processor {
    data: Vec<u8>,
}

impl Processor {
    pub unsafe fn append(&mut self, item: u8) {
        self.data.push(item);
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

pub fn simulate_input(input: &[u8], proc: &Arc<Mutex<Processor>>) -> Result<(), &'static str> {
    for &byte in input {
        unsafe {
            proc.lock().unwrap().append(byte);
        }
    }
    Ok(())
}

fn main() {
    let proc = Arc::new(Mutex::new(Processor { data: Vec::with_capacity(1024) }));
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let input = args[1].as_bytes();
        let _ = simulate_input(input, &proc);
    }
    loop {
        thread::sleep(Duration::from_millis(100));
    }
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
