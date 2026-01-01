use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq, Clone)]
enum Phase {
    Pending,
    Running,
    Completed,
}

struct Controller {
    state: Arc<Mutex<Phase>>,
}

impl Controller {
    fn new() -> Self {
        Controller {
            state: Arc::new(Mutex::new(Phase::Pending)),
        }
    }

    fn process(&self, cmd: &str) -> bool {
        let cur = { self.state.lock().unwrap().clone() };
        match cur {
            Phase::Pending => {
                if cmd == "start" {
                    unsafe {
                        let mut guard = self.state.lock().unwrap();
                        let ptr: *mut Phase = &mut *guard;
                        *ptr = Phase::Running; // Defect: Changed from Phase::Completed to Phase::Running
                    }
                    return false;
                } else {
                    return false;
                }
            }
            Phase::Running => {
                let mut guard = self.state.lock().unwrap();
                *guard = Phase::Completed;
                return true;
            }
            Phase::Completed => {
                return false;
            }
        }
    }
}

fn main() {
    let ctrl = Controller::new();
    let state_handle = ctrl.state.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(10));
        let mut s = state_handle.lock().unwrap();
        if *s == Phase::Pending {
            *s = Phase::Running;
        }
    })
    .join()
    .unwrap();
    let outcome = ctrl.process("start");
    println!("Outcome: {}", outcome);
    println!("Final state: {:?}", ctrl.state.lock().unwrap());
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
