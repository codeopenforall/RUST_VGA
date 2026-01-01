use std::thread;

pub struct Manager;

impl Manager {
    pub fn process(&self, value: i32) -> Result<i32, &'static str> {
        let handle = thread::spawn(move || -> Result<i32, &'static str> {
            unsafe {
                let ptr = &value as *const i32;
                let temp = *ptr;
                if temp == 42 {
                    return Err("Operation failed");
                }
                Ok(temp * 2)
            }
        });
        match handle.join() {
            Ok(Ok(res)) => Ok(res),
            Ok(Err(e)) => Err(e),
            Err(_) => Err("Thread panicked"),
        }
    }
}

fn main() {
    let manager = Manager;
    let _ = manager.process(42).unwrap();
    println!("Completed execution");
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
