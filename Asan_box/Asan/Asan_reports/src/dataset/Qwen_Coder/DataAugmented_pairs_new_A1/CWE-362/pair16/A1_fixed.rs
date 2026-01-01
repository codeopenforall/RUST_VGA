use std::cell::UnsafeCell;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct Core {
    flag: UnsafeCell<bool>,
    lock: Mutex<()>,
}

impl Core {
    pub fn new() -> Self {
        Self {
            flag: UnsafeCell::new(true),
            lock: Mutex::new(()),
        }
    }

    pub fn try_use(&self) -> bool {
        let _guard = self.lock.lock().unwrap(); // Acquire the lock before checking the flag
        unsafe {
            if *self.flag.get() {
                thread::sleep(Duration::from_micros(10));
                *self.flag.get() = false;
                return true;
            }
        }
        false
    }
}

unsafe impl Sync for Core {}

fn main() {
    let core = Arc::new(Core::new());
    let handles: Vec<_> = (0..10)
        .map(|_| {
            let core = core.clone();
            thread::spawn(move || {
                if core.try_use() {
                    println!("Resource used");
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::Core;
    use std::sync::Arc;
    use std::thread;

    // The test repeatedly spawns multiple threads attempting to use the resource.
    // It asserts that exactly one thread succeeds in using it.
    // For the vulnerable version, the race condition may cause the test to fail by allowing multiple successes.
    // For the fixed version, the test passes reliably.
    #[test]
    fn test_unique_access() {
        let iterations = 100;
        for _ in 0..iterations {
            let core = Arc::new(Core::new());
            let handles: Vec<_> = (0..50)
                .map(|_| {
                    let core = core.clone();
                    thread::spawn(move || {
                        // Convert the boolean result to an integer (1 for success, 0 otherwise)
                        core.try_use() as u32
                    })
                })
                .collect();

            let mut count = 0;
            for handle in handles {
                count += handle.join().unwrap();
            }
            assert_eq!(
                count, 1,
                "Expected exactly one successful access, but got {}",
                count
            );
        }
    }
}
