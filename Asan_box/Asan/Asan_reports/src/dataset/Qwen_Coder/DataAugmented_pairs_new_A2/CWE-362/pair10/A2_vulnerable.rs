use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Shared {
    counter: UnsafeCell<u32>,
}

unsafe impl Sync for Shared {}

impl Shared {
    fn new() -> Self {
        Shared {
            counter: UnsafeCell::new(0),
        }
    }
}

fn run_app() -> u32 {
    let shared = Arc::new(Shared::new());
    let mut handles = Vec::new();
    let iterations_per_thread = 1000;
    for _ in 0..10 {
        let data = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            for _ in 0..iterations_per_thread {
                unsafe {
                    let current = *data.counter.get();
                    if current < 10000 {
                        thread::sleep(Duration::from_micros(1));
                        *data.counter.get() = current + 1;
                    }
                }
            }
        });
        handles.push(handle);
    }
    for h in handles {
        h.join().expect("Thread panicked");
    }
    unsafe { *shared.counter.get() }
}

fn main() {
    let result = run_app();
    println!("Final counter: {}", result);
}

#[cfg(test)]
mod tests {
    // Import the run_app function from the library module.
    // In practice, ensure that both the vulnerable and fixed variants expose a common run_app interface.
    // For demonstration, we assume the fixed variant is being tested.
    use super::run_app;

    #[test]
    fn test_final_counter() {
        let result = run_app();
        // The contract requires the final counter value to precisely equal 10000.
        // In the vulnerable version, race conditions will cause the value to be lower than 10000,
        // causing this test to fail. The fixed version passes as it properly synchronizes access.
        assert_eq!(result, 10000, "Final counter does not meet expected value due to race condition");
    }
}
