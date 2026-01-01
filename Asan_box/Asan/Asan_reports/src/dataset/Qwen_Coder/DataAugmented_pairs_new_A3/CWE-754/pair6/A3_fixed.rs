use std::sync::atomic::{AtomicBool, Ordering};

struct Context {
    cancel_flag: AtomicBool,
    timeout_flag: AtomicBool,
}

impl Context {
    fn new(cancel: bool, timeout: bool) -> Self {
        Context {
            cancel_flag: AtomicBool::new(cancel),
            timeout_flag: AtomicBool::new(timeout),
        }
    }

    pub fn execute(&self) -> Result<i32, &'static str> {
        if self.timeout_flag.load(Ordering::SeqCst) {
            return Err("Operation timed out");
        }
        if self.cancel_flag.load(Ordering::SeqCst) {
            return Err("Operation cancelled");
        }
        unsafe {
            let raw_ptr = Box::into_raw(Box::new(256));
            let result = *raw_ptr;
            Box::from_raw(raw_ptr);
            Ok(result)
        }
    }
}

#[cfg(test)]
mod tests {
    // This test oracle is designed to work with both implementations.
    // It asserts that when the context is initialized with cancel=false and timeout=true,
    // the execute() method must fail. In the vulnerable version the method erroneously returns Ok.
    use super::*;

    #[test]
    fn test_exceptional_condition() {
        // Setup: cancellation is false but timeout is true.
        let ctx = Context::new(false, true);
        let result = ctx.execute();
        // The proper behavior (fixed version) is to return an error due to timeout.
        // The vulnerable version will incorrectly return Ok(256).
        assert!(
            result.is_err(),
            "Expected execution to fail due to timeout, but it succeeded."
        );
        if let Err(e) = result {
            assert_eq!(e, "Operation timed out", "Unexpected error message.");
        }
    }
}
