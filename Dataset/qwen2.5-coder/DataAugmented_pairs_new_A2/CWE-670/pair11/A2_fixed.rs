use std::sync::{Arc, Mutex};
use std::thread;

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
            let mut state = state_handle.lock().unwrap();
            if input > 0 {
                if *state == State::Init {
                    *state = State::Running;
                }
            } else {
                *state = State::Completed;
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