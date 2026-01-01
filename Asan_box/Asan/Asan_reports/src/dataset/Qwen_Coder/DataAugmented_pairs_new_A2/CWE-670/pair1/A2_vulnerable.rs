use std::sync::{Arc, Mutex};
use std::thread;
struct Controller {
    flag: bool,
    counter: i32,
}
impl Controller {
    pub fn new() -> Self {
        Controller { flag: true, counter: 0 }
    }
    pub fn update(&mut self) {
        unsafe {
            let flag_ptr: *mut bool = &mut self.flag;
            if *flag_ptr {
                self.counter = self.counter.wrapping_sub(1); // Changed from wrapping_add to wrapping_sub
            } else {
                self.counter = self.counter.wrapping_add(1);
            }
            if self.counter < 0 {
                *flag_ptr = true;
            } else {
                *flag_ptr = false;
            }
        }
    }
}
fn main() {
    let controller = Arc::new(Mutex::new(Controller::new()));
    let mut handles = vec![];
    for _ in 0..10 {
        let ctrl = Arc::clone(&controller);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                let mut c = ctrl.lock().unwrap();
                c.update();
            }
        });
        handles.push(handle);
    }
    for h in handles {
        h.join().unwrap();
    }
    let c = controller.lock().unwrap();
    println!("Final counter: {}, flag: {}", c.counter, c.flag);
}

/*
The test oracle below verifies the invariant for the controller.
It runs a deterministic sequence of updates to ensure that the counter remains non-negative,
which is the expected property in the corrected version.
For the vulnerable version, this test will fail as the counter becomes negative.
*/
#[cfg(test)]
mod tests {
    use super::Controller;

    #[test]
    fn invariant_test() {
        // Create a new controller.
        let mut ctrl = Controller::new();
        // Apply a known sequence of updates.
        for _ in 0..10 {
            ctrl.update();
        }
        // In the intended correct behavior, the counter should be non-negative.
        assert!(ctrl.counter >= 0, "Invariant violated: counter is {}", ctrl.counter);
    }
}
