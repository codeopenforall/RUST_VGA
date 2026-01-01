use std::sync::{Arc, Mutex};
use std::thread;

unsafe fn external_call(input: i32) -> i32 {
    if input > 0 {
        0
    } else {
        -1
    }
}

struct Operator {
    value: i32,
}

impl Operator {
    fn new(val: i32) -> Self {
        Operator { value: val }
    }

    fn execute(&self) -> bool {
        let ret = unsafe { external_call(self.value) };
        ret == 0
    }
}

fn main() {
    let op = Arc::new(Mutex::new(Operator::new(5)));
    let mut threads = vec![];

    for _ in 0..5 {
        let op_clone = Arc::clone(&op);
        let handle = thread::spawn(move || {
            let instance = op_clone.lock().unwrap();
            if instance.execute() {
                println!("Operation succeeded");
            } else {
                println!("Operation failed");
            }
        });
        threads.push(handle);
    }

    for handle in threads {
        handle.join().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test that should succeed with the corrected version and fail with the vulnerable version.
    // For the input value of 5, external_call is designed to return 0 for success.
    // Hence, execute() should return true.
    #[test]
    fn test_operator_execution() {
        let op = Operator::new(5);
        // Assert that the operation indicates success.
        assert!(op.execute(), "Expected the operation to succeed with a valid input");
    }
}
