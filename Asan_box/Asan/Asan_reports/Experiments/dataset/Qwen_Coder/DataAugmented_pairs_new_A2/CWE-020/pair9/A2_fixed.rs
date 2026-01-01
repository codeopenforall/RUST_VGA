use std::error::Error;
use std::str;
use std::sync::{Arc, Mutex};
use std::thread;

struct Engine {
    info: Arc<Mutex<String>>,
}

impl Engine {
    fn new(initial: &str) -> Self {
        Self {
            info: Arc::new(Mutex::new(initial.to_owned())),
        }
    }

    fn execute(&self, input: &str) -> Result<String, Box<dyn Error>> {
        let len: usize = input.trim().parse()?;
        let guard = self.info.lock().unwrap();
        let bytes = guard.as_bytes();
        if len > bytes.len() {
            return Err("Input length exceeds available data".into());
        }
        let snippet = unsafe { str::from_utf8_unchecked(&bytes[0..len]) };
        Ok(snippet.to_string())
    }

    fn parallel_run(&self, input: &str) -> Result<String, Box<dyn Error>> {
        let len_val = input.trim().parse::<usize>()?;
        let shared = Arc::clone(&self.info);
        let handler = thread::spawn(move || {
            let locked = shared.lock().unwrap();
            let bytes = locked.as_bytes();
            if len_val > bytes.len() {
                return "Error: Length exceeds data".to_string();
            }
            let segment = unsafe { str::from_utf8_unchecked(&bytes[0..len_val]) };
            segment.to_string()
        });
        let result = handler.join().map_err(|_| "Thread join error")?;
        if result == "Error: Length exceeds data" {
            return Err("Input length exceeds available data".into());
        }
        Ok(result)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let system = Engine::new("trusted_data");
    let user_length = "5";
    let outcome = system.execute(user_length)?;
    println!("Outcome: {}", outcome);
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
