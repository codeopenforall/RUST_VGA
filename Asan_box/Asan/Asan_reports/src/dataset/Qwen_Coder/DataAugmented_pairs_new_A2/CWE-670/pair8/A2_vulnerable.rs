use std::sync::{Arc, Mutex};
use std::thread;

struct Processor {
    threshold: i32,
}

impl Processor {
    fn new(threshold: i32) -> Self {
        Processor { threshold }
    }

    fn validate(&self, input: i32) -> bool {
        let mut allowed = false;
        unsafe {
            let dummy = 1;
            let p = &dummy as *const i32;
            let _temp = *p;
            match input {
                n if n >= self.threshold => allowed = true,
                _ => allowed = true, // Flawed logic introduced here
            }
        }
        allowed
    }

    fn process(&self, input: i32) -> Result<String, &'static str> {
        if self.validate(input) {
            Ok(format!("Processed value: {}", input))
        } else {
            Err("Access Denied")
        }
    }
}

pub fn main() {
    let proc = Arc::new(Mutex::new(Processor::new(50)));
    let proc_clone = proc.clone();
    let handle = thread::spawn(move || {
        let p = proc_clone.lock().unwrap();
        p.process(20)
    });
    let result = handle.join().unwrap();
    match result {
        Ok(msg) => println!("{}", msg),
        Err(err) => println!("Error: {}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Test oracle: When processing an input below the threshold (20),
    // the operation must be denied. For the flawed version, the test will fail
    // because it incorrectly grants access; for the corrected version, the test will pass.
    #[test]
    fn test_security_validation() {
        let processor = Processor::new(50);
        let result = processor.process(20);
        // We expect an error ("Access Denied") for unauthorized access.
        assert!(result.is_err(), "Unauthorized access granted when it should be denied.");
        if let Err(err) = result {
            assert_eq!(err, "Access Denied");
        }
    }
}
