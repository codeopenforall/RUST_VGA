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