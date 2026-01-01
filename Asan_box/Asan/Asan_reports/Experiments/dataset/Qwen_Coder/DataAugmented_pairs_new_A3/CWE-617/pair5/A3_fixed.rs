use std::sync::Arc;
use std::thread;

struct Locker {
    data: Arc<Vec<u64>>,
}

impl Locker {
    fn new() -> Self {
        Locker { data: Arc::new(vec![0; 10]) }
    }

    fn update(&self, index: usize, value: u64) {
        if index >= self.data.len() {
            eprintln!("Index out of range: {}", index);
            return;
        }
        unsafe {
            let ptr = self.data.as_ptr() as *mut u64;
            *ptr.add(index) = value;
        }
    }
}

fn main() {
    let locker = Locker::new();
    let locker_clone = locker;
    let handle = thread::spawn(move || {
        locker_clone.update(15, 42);
    });
    let _ = handle.join();
    println!("Execution completed.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn test_update_behavior() {
        let locker = Locker::new();
        // Simulate attacker-controlled input with an out-of-bound index.
        let result = panic::catch_unwind(|| {
            // In the vulnerable version, this call would panic,
            // while in the corrected version it returns an error.
            // To ensure no panic occurs, we call update and ignore its return value.
            let _ = locker.update(15, 42);
        });
        assert!(result.is_ok(), "The update function panicked, indicating a vulnerability.");
    }
}
