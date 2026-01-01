use std::sync::{Arc, Mutex};

struct Engine {
    data: Arc<Mutex<Vec<i32>>>,
}

impl Engine {
    fn new() -> Self {
        Engine {
            data: Arc::new(Mutex::new(vec![0; 10])),
        }
    }

    pub fn run(&self, index: usize) {
        let mut vec = self.data.lock().unwrap();
        if index >= vec.len() {
            eprintln!("Error: index {} out-of-bounds", index);
            return;
        }

        vec[index] = 40;

        if index < vec.len() {
            unsafe {
                let ptr = vec.as_ptr();
                let value = *ptr.add(index);
                if value >= 50 {
                    eprintln!("Error: value {} at index {} out of acceptable range", value, index);
                    return;
                }
                println!("Value at index {} is acceptable: {}", index, value);
            }
        }
    }
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
