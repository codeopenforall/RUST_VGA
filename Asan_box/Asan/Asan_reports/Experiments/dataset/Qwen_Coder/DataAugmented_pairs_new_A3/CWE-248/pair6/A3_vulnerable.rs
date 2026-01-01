use std::thread;

struct Manager;

impl Manager {
    pub fn process(&self, value: i32) -> Result<i32, String> {
        let handle = thread::spawn(move || {
            if value == 42 {
                panic!("Unexpected value");
            }
            value * 2
        });

        let res = handle.join().unwrap();
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;
    
    #[test]
    fn test_exception_handling() {
        let mgr = Manager;
        // For the vulnerable version, invoking process(42) and unwrapping its result
        // should lead to a panic. We use catch_unwind to verify that.
        let result = panic::catch_unwind(|| {
            // In the vulnerable build, this will cause an uncaught panic.
            mgr.process(42).unwrap();
        });
        assert!(result.is_err(), "Expected panic in the vulnerable version");
        
        // In the fixed version, process(42) should not panic but rather return an error.
        let fixed_result = mgr.process(42);
        assert!(fixed_result.is_err(), "Expected error in the fixed version");
        assert_eq!(fixed_result.err().unwrap(), "Operation failed");
    }
}
