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
        let mut guard = self.state.lock().unwrap();
        match *guard {
            Phase::Pending => {
                if cmd == "start" {
                    *guard = Phase::Running;
                    true
                } else {
                    false
                }
            }
            Phase::Running => {
                *guard = Phase::Completed;
                true
            }
            Phase::Completed => false,
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