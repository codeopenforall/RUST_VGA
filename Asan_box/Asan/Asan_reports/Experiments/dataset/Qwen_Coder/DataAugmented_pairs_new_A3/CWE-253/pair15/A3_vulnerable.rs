use std::sync::{Arc, Mutex};
use std::thread;

struct Container {
    number: i32,
}

impl Container {
    unsafe fn try_update(&mut self, new_number: i32) -> Result<(), &'static str> {
        if new_number == 0 {
            return Err("Zero is not allowed");
        }
        self.number = new_number;
        Ok(())
    }
}

fn concurrent_update(shared: Arc<Mutex<Container>>, new_number: i32) -> bool {
    let mut threads = vec![];
    for _ in 0..4 {
        let data = shared.clone();
        threads.push(thread::spawn(move || {
            unsafe {
                let mut guard = data.lock().unwrap();
                let ret = guard.try_update(new_number);
                if ret.is_err() {
                    true
                } else {
                    false
                }
            }
        }));
    }
    let mut success = false;
    for handle in threads {
        success |= handle.join().unwrap();
    }
    success
}

fn main() {
    let shared = Arc::new(Mutex::new(Container { number: 42 }));
    if concurrent_update(shared.clone(), 0) {
        println!("Concurrent update reported success (vulnerable behavior).");
    } else {
        println!("Concurrent update reported failure.");
    }
}

//////////////////////
// Test Oracle Function
//////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    // The test oracle checks that when provided an invalid value (0),
    // the concurrent update function reports failure.
    // For the vulnerable version, the test should fail because it erroneously reports success.
    // For the fixed version, the test should pass.
    #[test]
    fn contract_check() {
        let shared = Arc::new(Mutex::new(Container { number: 100 }));
        // Use an invalid value to trigger the error.
        let result = concurrent_update(shared.clone(), 0);
        // The contract: invalid update should NOT be reported as success.
        assert_eq!(result, false, "Update incorrectly reported as successful upon invalid input");
    }
}
