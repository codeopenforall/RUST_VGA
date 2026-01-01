use std::sync::{Arc, Mutex};
use std::thread;

pub struct Context {
    pub state: Arc<Mutex<Vec<i32>>>,
}

impl Context {
    pub fn new(size: usize) -> Self {
        Context {
            state: Arc::new(Mutex::new(vec![0; size])),
        }
    }

    pub fn update(&self, index: usize, value: i32) {
        let mut lock = self.state.lock().unwrap();
        unsafe {
            let ptr = lock.as_mut_ptr();
            *ptr.add(index) = value;
        }
    }

    pub fn verify(&self) {
        let lock = self.state.lock().unwrap();
        assert!(lock[0] >= 0, "Invariant violated: state[0] is negative");
    }
}

pub fn start() {
    let ctx = Context::new(10);
    let handle = {
        let shared = ctx.state.clone();
        thread::spawn(move || {
            let mut data = shared.lock().unwrap();
            unsafe {
                let ptr = data.as_mut_ptr();
                *ptr.add(0) = -999; // Introducing the vulnerability
            }
        })
    };
    handle.join().unwrap();
    ctx.verify(); // This will panic because the invariant is violated
    println!("Execution completed in the vulnerable configuration.");
}

fn main() {
    start();
}

//////////////////////////
// Test Oracle Function
//////////////////////////

// This test oracle is independent from the main application code.
// It verifies that running the application does not trigger the invariant's assertion.
// In the vulnerable configuration, the reachable assertion fires and causes a panic,
// so this test will fail; in the fixed configuration, the test passes.

#[cfg(test)]
mod tests {
    use std::panic;

    #[test]
    fn test_invariant_maintained() {
        // Call the exported start function and catch unwinding panics.
        // In the fixed version, start() returns Ok(()), whereas the vulnerable version panics.
        let result = panic::catch_unwind(|| {
            // Depending on which configuration is being tested,
            // this function call should either panic or run successfully.
            crate::start();
        });
        assert!(
            result.is_ok(),
            "Execution panicked: invariant check triggered a reachable assertion failure"
        );
    }
}
