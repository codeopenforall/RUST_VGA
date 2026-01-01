use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
#[derive(Debug, PartialEq)]
enum State {
    Init,
    Running,
    Completed,
}
struct Processor {
    state: Arc<Mutex<State>>,
}
impl Processor {
    fn new() -> Self {
        Processor {
            state: Arc::new(Mutex::new(State::Init)),
        }
    }
    fn process(&self, input: i32) -> bool {
        let state_handle = Arc::clone(&self.state);
        let handle = thread::spawn(move || {
            unsafe {
                let raw_ptr: *mut State = &mut *state_handle.lock().unwrap() as *mut State;
                if input > 0 {
                    match *raw_ptr {
                        State::Init => {
                            *raw_ptr = State::Completed; // Vulnerable: Incorrect state transition
                        },
                        _ => {
                        },
                    }
                } else {
                    *raw_ptr = State::Completed;
                }
            }
        });
        handle.join().unwrap();
        let current = self.state.lock().unwrap();
        *current == State::Running
    }
}
fn main() {
    let proc_inst = Processor::new();
    let outcome = proc_inst.process(10);
    println!("Processing outcome: {}", outcome);
}

#[cfg(test)]
mod tests {
    use super::*;

    // This test verifies that processing a positive value correctly transitions the state to Running.
    // The expectation is that the fixed implementation returns true, whereas the flawed one returns false.
    #[test]
    fn test_state_transition() {
        let processor = Processor::new();
        let result = processor.process(10);
        // In the correct version, a positive input should move the state from Init to Running.
        // Thus, the assertion expects a true outcome.
        assert_eq!(result, true, "Expected state transition to Running did not occur");
    }
}
