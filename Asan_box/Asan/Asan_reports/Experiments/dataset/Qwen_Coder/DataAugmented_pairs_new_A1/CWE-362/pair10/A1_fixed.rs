use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Shared {
    counter: AtomicU32,
}

impl Shared {
    fn new() -> Self {
        Shared {
            counter: AtomicU32::new(0),
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
                while data.counter.load(Ordering::SeqCst) < 10000 {
                    let current = data.counter.load(Ordering::SeqCst);
                    let next = current + 1;
                    if data.counter.compare_and_swap(current, next, Ordering::SeqCst) == current {
                        break;
                    }
                }
            }
        });
        handles.push(handle);
    }
    for h in handles {
        h.join().expect("Thread panicked");
    }
    shared.counter.load(Ordering::SeqCst)
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
