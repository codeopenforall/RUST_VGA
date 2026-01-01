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