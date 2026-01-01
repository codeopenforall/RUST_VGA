use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;

struct ExecutionState {
    halt_signal: Arc<AtomicBool>,
    time_exceeded: Arc<AtomicBool>,
}

impl ExecutionState {
    fn initialize(halt: bool, time_limit: bool) -> Self {
        ExecutionState {
            halt_signal: Arc::new(AtomicBool::new(halt)),
            time_exceeded: Arc::new(AtomicBool::new(time_limit)),
        }
    }

    fn perform(&self) -> Result<i32, &'static str> {
        if !self.halt_signal.load(Ordering::SeqCst) {
            unsafe {
                let raw_box = Box::into_raw(Box::new(256));
                let outcome = *raw_box;
                Box::from_raw(raw_box);
                Ok(outcome)
            }
        } else {
            Err("Operation cancelled")
        }
    }
}

fn main() {
    let state = ExecutionState::initialize(false, true);
    match state.perform() {
        Ok(value) => println!("Success: Result is {}", value),
        Err(error) => println!("Error: {}", error),
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
