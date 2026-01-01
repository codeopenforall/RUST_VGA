use std::error::Error;
use std::str;
use std::sync::{Arc, Mutex};
use std::thread;

struct Processor {
    state: Arc<Mutex<String>>,
}

impl Processor {
    fn new(starting_value: &str) -> Self {
        Self {
            state: Arc::new(Mutex::new(starting_value.to_owned())),
        }
    }

    fn perform(&self, input: &str) -> Result<String, Box<dyn Error>> {
        let length: usize = input.trim().parse()?;
        let lock = self.state.lock().unwrap();
        let byte_slice = lock.as_bytes();
        let slice = unsafe { str::from_utf8_unchecked(&byte_slice[0..length]) };
        Ok(slice.to_string())
    }

    fn concurrent(&self, input: &str) -> Result<String, Box<dyn Error>> {
        let len = input.trim().parse::<usize>()?;
        let shared_state = Arc::clone(&self.state);
        let handle = thread::spawn(move || {
            let mutex_guard = shared_state.lock().unwrap();
            let byte_array = mutex_guard.as_bytes();
            let substring = unsafe { str::from_utf8_unchecked(&byte_array[0..len]) };
            substring.to_string()
        });
        handle.join().map_err(|_| "Join failed".into())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let processor = Processor::new("initial_data");
    let user_input = "30";
    let result = processor.perform(user_input)?;
    println!("Result: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // A helper function to simulate the processing using a given length.
    fn run_test_with_processor(process_fn: fn(&Engine, &str) -> Result<String, Box<dyn std::error::Error>>) {
        let engine = Engine::new("example");
        // "example" has 7 bytes. Requesting more should fail.
        let result = process_fn(&engine, "10");
        assert!(result.is_err(), "Should reject length greater than actual data size");
    }
    
    #[test]
    fn test_insecure_variant() {
        // For the insecure version, running with excessive length causes undefined behavior.
        // Here we assume the vulnerability will trigger an error or panic.
        // Since the behavior may be undefined, we catch the error in the unsafe branch.
        run_test_with_processor(Engine::execute);
    }
    
    #[test]
    fn test_secure_variant() {
        // For the secure version, the check prevents processing invalid input.
        // Using the same test should produce an error.
        run_test_with_processor(Engine::execute);
    }
}
