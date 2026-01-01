use std::sync::{Arc, Mutex};
use std::thread;

struct Engine {
    data: Arc<Mutex<Vec<u32>>>,
}

impl Engine {
    fn new() -> Self {
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        Engine {
            data: Arc::new(Mutex::new(vec)),
        }
    }

    fn run(&self, index: usize) {
        let data_clone = Arc::clone(&self.data);
        let idx = index;
        let handle = thread::spawn(move || {
            let mut vec = data_clone.lock().unwrap();
            if idx < vec.len() && idx % 2 == 0 {
                vec[idx] = 100; // Intentional defect: modifying the vector element
            }
        });
        handle.join().unwrap();
        let vec = self.data.lock().unwrap();
        unsafe {
            let ptr = vec.as_ptr();
            let value = *ptr.add(index);
            assert!(value < 50, "Assertion triggered by unexpected state"); // This assertion will fail
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(0)
    } else {
        0
    };
    let engine = Engine::new();
    engine.run(input);
}

////////////////////////////////////////
// Test Oracle for Reachable Assertion  //
////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::panic;
    // Bring the Engine struct into scope.
    use super::*;

    #[test]
    fn test_reachable_assertion() {
        // Using an even index (2) that is within bounds.
        // In the vulnerable version, the spawned thread sets the value to 100,
        // causing the assertion to fail and thus panicking.
        // In the corrected version, the value is modified to 40, keeping the invariant intact.
        let engine = Engine::new();
        let result = panic::catch_unwind(|| {
            engine.run(2);
        });
        // The test asserts that the Engine::run call does not panic.
        // This oracle will fail when testing the vulnerable version and pass when testing the fix.
        assert!(result.is_ok(), "Engine::run should not panic for index 2");
    }
}
