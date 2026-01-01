use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, PartialEq)]
enum Mode {
    Idle,
    Active,
}

struct Controller {
    mode: Mode,
}

impl Controller {
    fn new() -> Controller {
        Controller { mode: Mode::Idle }
    }

    fn update(&mut self, trigger: bool) -> bool {
        unsafe {
            match self.mode {
                Mode::Idle if trigger => {
                    let new_state = Mode::Idle; // Defect: Incorrectly setting new_state to Mode::Idle instead of Mode::Active
                    self.mode = new_state;
                    true
                }
                Mode::Active if !trigger => {
                    self.mode = Mode::Idle;
                    false
                }
                _ => false,
            }
        }
    }
}

fn main() {
    let controller = Arc::new(Mutex::new(Controller::new()));
    let ctrl_clone = Arc::clone(&controller);
    let handle = thread::spawn(move || {
        let mut ctrl = ctrl_clone.lock().unwrap();
        let result = ctrl.update(true);
        if result {
            println!("Transition succeeded");
        } else {
            println!("Transition failed");
        }
    });
    handle.join().unwrap();
    let ctrl_final = controller.lock().unwrap();
    if ctrl_final.mode == Mode::Active {
        println!("State is active");
    } else {
        println!("State is idle");
    }
}

#[cfg(test)]
mod tests {
    // Bring the items from the parent module into scope.
    use super::*;

    #[test]
    fn test_state_transition() {
        // Initialize the controller in Idle state.
        let mut ctl = Controller::new();
        // Trigger an update expecting a transition to Active.
        let result = ctl.update(true);

        // For the vulnerable version, result will be true but the state remains Idle.
        // For the fixed version, the state will correctly become Active.
        assert!(result, "Expected update(true) to return success");
        assert_eq!(
            ctl.mode,
            Mode::Active,
            "Expected Controller to be in Active state after update(true)"
        );
    }
}
