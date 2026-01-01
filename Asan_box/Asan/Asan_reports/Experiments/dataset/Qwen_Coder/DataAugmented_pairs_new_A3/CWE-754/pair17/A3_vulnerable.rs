use std::thread;
use std::time::Duration;
#[derive(Debug, PartialEq)]
enum ProcError {
    Timeout,
    Other,
}
struct Worker;
impl Worker {
    fn new() -> Self {
        Worker {}
    }
    fn process(&self, val: i32) -> Result<*mut i32, ProcError> {
        let handle = thread::spawn(move || {
            if val % 2 == 0 {
                Box::new(val * 2)
            } else {
                thread::sleep(Duration::from_millis(50));
                panic!("Operation timed out");
            }
        });
        match handle.join() {
            Ok(data) => Ok(Box::into_raw(data)),
            Err(_) => Err(ProcError::Timeout),
        }
    }
    fn execute(&self, val: i32) -> Result<i32, ProcError> {
        match self.process(val) {
            Ok(ptr) => {
                let res = unsafe { *ptr };
                unsafe { Box::from_raw(ptr) };
                Ok(res)
            },
            Err(e) => {
                if let ProcError::Timeout = e {
                    Ok(100) // Vulnerable: Returning a default value instead of propagating the error
                } else {
                    Ok(0)
                }
            }
        }
    }
}
fn main() {
    let worker = Worker::new();
    match worker.execute(1) {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {:?}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exceptional_condition() {
        let worker = Worker::new();
        // For input 1 (an odd number), the process is designed to time out.
        // In the fixed version, execute returns an error.
        // In the vulnerable version, it erroneously returns a default value (100).
        let result = worker.execute(1);
        match result {
            Err(ProcError::Timeout) => assert!(true, "Proper error propagated"),
            Ok(val) => panic!("Test failed: Expected error, but got success with value: {}", val),
            _ => panic!("Test failed: Unexpected result variant"),
        }
    }
}
