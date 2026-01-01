use std::sync::{Arc, Mutex};

#[derive(Debug, PartialEq)]
enum Phase {
    Idle,
    Running,
    Completed,
}

pub struct Controller {
    state: Arc<Mutex<Phase>>,
}

impl Controller {
    pub fn new() -> Self {
        Controller {
            state: Arc::new(Mutex::new(Phase::Idle)),
        }
    }

    pub fn process(&self, command: &str) -> bool {
        if command == "start" {
            unsafe {
                let mut guard = self.state.lock().unwrap();
                let ptr: *mut Phase = &mut *guard;
                *ptr = Phase::Completed;
            }
            return false;
        }
        true
    }
}

/////////////////////// Test Oracle ///////////////////////
#[cfg(test)]
mod test_oracle {
    use super::Controller;

    // The oracle function tests that processing a valid "start" command returns true.
    // For the vulnerable version, this test would fail (returning false); for the fixed version, it passes.
    #[test]
    fn command_processing_oracle() {
        let ctrl = Controller::new();
        let result = ctrl.process("start");
        // The expected behavior is that with a valid command, the state should be updated properly.
        assert!(result, "Expected processing a valid command to succeed.");
    }
}
