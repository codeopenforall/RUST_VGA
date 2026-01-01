use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;
pub struct Data {
    counter: UnsafeCell<u32>,
}
unsafe impl Sync for Data {}
impl Data {
    pub fn new(val: u32) -> Data {
        Data {
            counter: UnsafeCell::new(val),
        }
    }
    pub fn decrease(&self, sub: u32) -> Result<u32, &'static str> {
        unsafe {
            let current = *self.counter.get();
            let new_val = current.wrapping_sub(sub); // Vulnerable: using wrapping_sub
            *self.counter.get() = new_val;
            Ok(new_val)
        }
    }
}
pub fn process_operation(initial: u32, sub: u32) -> Result<u32, &'static str> {
    let data = Data::new(initial);
    let shared_data = Arc::new(data);
    let thread_data = Arc::clone(&shared_data);
    let handle = thread::spawn(move || {
        thread_data.decrease(sub)
    });
    handle.join().unwrap()
}
fn main() {
    let res = process_operation(100, 200);
    match res {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}

//////////////////////////////////////////////////////////////
// Test Oracle: This unit test verifies that using an initial value
// of 100 and subtracting 200 should trigger an error condition in
// the corrected implementation, but not in the vulnerable one.
// The test is designed to fail for the vulnerable version and pass
// for the fixed version.
//////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    // Import the process_operation function from the module.
    use super::process_operation;

    #[test]
    fn test_underflow_behavior() {
        let initial = 100u32;
        let subtractor = 200u32;
        let result = process_operation(initial, subtractor);
        
        // In the fixed version, the subtraction should trigger an underflow error.
        // Thus, we expect the result to be an error.
        assert!(
            result.is_err(),
            "Expected error due to integer underflow, but got: {:?}",
            result.ok()
        );
    }
}
