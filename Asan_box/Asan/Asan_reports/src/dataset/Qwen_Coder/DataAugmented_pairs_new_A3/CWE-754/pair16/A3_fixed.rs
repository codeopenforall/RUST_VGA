#![allow(unused)]
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct OperationResult {
    is_timeout: bool,
    is_cancelled: bool,
    value: i32,
}

trait Processor {
    fn process(&self) -> Result<i32, &'static str>;
}

struct Worker {
    data: i32,
}

impl Processor for Worker {
    fn process(&self) -> Result<i32, &'static str> {
        let op = simulate_operation();
        if op.is_cancelled {
            return Err("Timeout occurred");
        }
        let mut local = op.value;
        let shared = Arc::new(Mutex::new(local));
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            let mut val = shared_clone.lock().unwrap();
            *val += 1;
        });
        handle.join().unwrap();
        let final_val = *shared.lock().unwrap();
        Ok(final_val)
    }
}

fn simulate_operation() -> OperationResult {
    OperationResult {
        is_timeout: true,
        is_cancelled: true,
        value: 42,
    }
}

fn main() {
    let worker = Worker { data: 10 };
    match worker.process() {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_exception_handling() {
        // Instantiate the worker.
        let worker = Worker { data: 10 };
        // Trigger the processing function which should detect the timeout exception.
        let result = worker.process();
        // For the vulnerable implementation this test would erroneously pass,
        // but for the fixed implementation it correctly errors.
        assert!(result.is_err(), "Expected error due to timeout condition");
        if let Err(msg) = result {
            assert_eq!(msg, "Timeout occurred");
        }
    }
}
