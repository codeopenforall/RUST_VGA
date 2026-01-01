use std::sync::{Arc, Mutex};
use std::thread;
#[derive(Clone, Copy, Debug, PartialEq)]
enum Access {
    Allowed,
    Blocked,
}
struct Handler {
    flag: Arc<Mutex<Access>>,
}
impl Handler {
    fn new() -> Self {
        Handler {
            flag: Arc::new(Mutex::new(Access::Blocked)),
        }
    }
    fn update(&self, input: i32) {
        let new_state = match input > 100 {
            true => Access::Allowed,
            false => Access::Blocked,
        };
        let mut current = self.flag.lock().unwrap();
        *current = new_state;
    }
    fn check(&self) -> bool {
        let current = self.flag.lock().unwrap();
        match *current {
            Access::Allowed => true,
            Access::Blocked => false,
        }
    }
}
fn main() {
    let handler = Handler::new();
    let shared_handler = Arc::new(handler);
    let mut handles = vec![];
    for _ in 0..5 {
        let clone = Arc::clone(&shared_handler);
        let t = thread::spawn(move || {
            clone.update(150);
        });
        handles.push(t);
    }
    for t in handles {
        t.join().unwrap();
    }
    if shared_handler.check() {
        println!("Access granted");
    } else {
        println!("Access denied");
    }
}

//
// Test oracle function to verify the vulnerability fix.
// This function should be placed in a separate tests module and is not part of the main binaries.
//
// Note: The test uses a single-threaded call to simplify validation of the update and check functions.
// For the vulnerable version, the test will fail as the incorrect control flow returns a denied state.
// For the fixed version, the test passes.
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_control() {
        let handler = Handler::new();
        // Triggering input (150) should grant access.
        handler.update(150);
        assert!(handler.check(), "Access should be granted for input 150");
    }
}
