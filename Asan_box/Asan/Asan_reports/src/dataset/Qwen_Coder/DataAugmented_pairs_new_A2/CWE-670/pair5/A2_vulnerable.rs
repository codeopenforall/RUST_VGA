use std::sync::{Arc, Mutex};
use std::thread;

struct Controller {
    counter: Arc<Mutex<i32>>,
}

impl Controller {
    fn new(initial: i32) -> Self {
        Controller {
            counter: Arc::new(Mutex::new(initial)),
        }
    }

    fn update(&self, adjustment: i32) -> Result<i32, &'static str> {
        let shared = Arc::clone(&self.counter);
        let handle = thread::spawn(move || {
            let mut num = shared.lock().unwrap();
            unsafe {
                *num = num.wrapping_add(adjustment);
            }
        });
        handle.join().unwrap();
        let current = *self.counter.lock().unwrap();
        if current < 0 {
            Ok(current)
        } else {
            Err("Invalid state: non-negative counter")
        }
    }
}

fn main() {
    let controller = Controller::new(1);
    match controller.update(10) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}

//////////////// Test Oracle Begin
// This test oracle is designed to run against both compiled versions.
// For the provided input (initial value 1 and adjustment +10), the expected outcome is a valid non-negative result.
// The vulnerable version incorrectly returns an error, causing this test to fail.
// The fixed version returns Ok(result) and the test passes.
#[cfg(test)]
mod tests {
    use super::Controller;
    
    #[test]
    fn check_control_flow() {
        // Instantiate with a valid initial state.
        let ctrl = Controller::new(1);
        // With an adjustment of +10, the final counter should be 11.
        let res = ctrl.update(10);
        match res {
            Ok(val) => assert_eq!(val, 11, "Expected valid result of 11"),
            Err(err) => panic!("Test failed due to control flow error: {}", err),
        }
    }
}
//////////////// Test Oracle End
