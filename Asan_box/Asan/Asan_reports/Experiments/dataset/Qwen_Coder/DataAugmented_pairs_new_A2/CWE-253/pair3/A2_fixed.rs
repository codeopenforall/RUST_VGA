use std::sync::{Arc, Mutex};
use std::thread;

unsafe fn external_process(value: i32) -> i32 {
    if value > 100 {
        1
    } else {
        0
    }
}

struct Engine {
    flag: Arc<Mutex<bool>>,
}

impl Engine {
    fn new() -> Self {
        Engine {
            flag: Arc::new(Mutex::new(false)),
        }
    }

    fn process(&self, value: i32) -> bool {
        let flag = self.flag.clone();
        let handle = thread::spawn(move || {
            let res = unsafe { external_process(value) };
            let mut f = flag.lock().unwrap();
            *f = res == 0; // Fix: Set flag to true if res is 0 (success), false otherwise (failure)
        });
        handle.join().unwrap();
        *self.flag.lock().unwrap()
    }
}

fn main() {
    let engine = Engine::new();
    let outcome = engine.process(150);
    if outcome {
        println!("Operation succeeded");
    } else {
        println!("Operation failed");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operation() {
        let engine = Engine::new();
        // For the given triggering input (150), the external_process returns 1 indicating an error.
        // The fixed behavior should return false (indicating failure), while the vulnerable version incorrectly returns true.
        let outcome = engine.process(150);
        assert_eq!(outcome, false, "Expected operation to fail for input > 100");
    }
}
