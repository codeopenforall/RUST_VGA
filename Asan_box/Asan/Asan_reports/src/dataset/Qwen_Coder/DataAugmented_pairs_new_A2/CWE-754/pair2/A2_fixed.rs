use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Shared {
    value: i32,
    flag: bool,
}

impl Shared {
    fn new(val: i32) -> Self {
        Shared { value: val, flag: false }
    }

    fn update(&mut self, delta: i32) -> Result<i32, &'static str> {
        if self.flag {
            return Err("error flag set");
        }
        self.value += delta;
        Ok(self.value)
    }
}

fn perform_task(shared: Arc<Mutex<Shared>>, delta: i32) -> Result<i32, &'static str> {
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(10));
        let mut lock = shared_clone.lock().unwrap();
        lock.flag = true;
    });
    handle.join().unwrap();
    let mut lock = shared.lock().unwrap();
    lock.update(delta)
}

fn main() {
    let shared = Arc::new(Mutex::new(Shared::new(10)));
    match perform_task(shared, 5) {
        Ok(val) => println!("Operation succeeded with result: {}", val),
        Err(err) => println!("Operation failed: {}", err),
    }
}

// Test Oracle Function for CWE-754
// This unit test is designed to fail against the vulnerable code and pass with the fixed code.
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_exception() {
        let shared = Arc::new(Mutex::new(Shared::new(10)));
        // When the error flag is set by the spawned thread, perform_task() should return an error.
        let result = perform_task(shared, 5);
        assert!(result.is_err(), "Expected an error due to the exception flag, but got success");
    }
}
