use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, PartialEq, Clone)]
enum Mode {
    Active,
    Inactive,
}

struct Processor {
    state: Arc<Mutex<Mode>>,
}

impl Processor {
    fn new() -> Self {
        Processor {
            state: Arc::new(Mutex::new(Mode::Inactive)),
        }
    }

    fn update(&self, cond: bool) -> u32 {
        let mut st = self.state.lock().unwrap();
        if cond {
            *st = Mode::Active;
            100
        } else {
            *st = Mode::Inactive;
            200
        }
    }

    fn get_state(&self) -> Mode {
        let st = self.state.lock().unwrap();
        (*st).clone()
    }

    fn run(&self, cond: bool) -> u32 {
        self.update(cond)
    }
}

fn main() {
    let proc = Processor::new();
    let proc_clone = proc.state.clone();
    let handle = thread::spawn(move || {
        let mut st = proc_clone.lock().unwrap();
        *st = Mode::Active;
    });
    handle.join().unwrap();
    let result = proc.run(true);
    println!("Result: {}", result);
    println!("State: {:?}", proc.get_state());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contract_test() {
        let proc = Processor::new();
        // Test with 'true': expected to set state to Active and return 100.
        let ret = proc.run(true);
        assert_eq!(ret, 100, "Return value should be 100 when condition is true");
        assert_eq!(proc.get_state(), Mode::Active, "State should be Active when condition is true");
    }
}
