use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Shared {
    counter: UnsafeCell<u32>,
}

unsafe impl Sync for Shared {}

fn run_app() -> u32 {
    let shared = Arc::new(Shared {
        counter: UnsafeCell::new(0),
    });

    let mut handles = vec![];

    for _ in 0..10 {
        let shared_clone = Arc::clone(&shared);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                unsafe {
                    let current = *shared_clone.counter.get();
                    if current < 10000 {
                        thread::sleep(Duration::from_micros(1));
                        *shared_clone.counter.get() = current + 1;
                    }
                }
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    unsafe { *shared.counter.get() }
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
