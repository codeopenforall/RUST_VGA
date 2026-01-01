use std::sync::atomic::{AtomicU32, Ordering};
use std::thread;

static GLOBAL: AtomicU32 = AtomicU32::new(0);

fn update(count: u32) {
    for _ in 0..count {
        GLOBAL.fetch_add(1, Ordering::SeqCst);
    }
}

fn run() -> u32 {
    let mut handles = Vec::new();
    for _ in 0..10 {
        handles.push(thread::spawn(|| {
            update(1000);
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    GLOBAL.load(Ordering::SeqCst)
}

fn main() {
    let result = run();
    println!("Final count: {}", result);
}

/////////////////// Test Oracle ///////////////////
// This oracle function is designed to be used in a unit test environment.
// It runs the shared counter update routine and asserts that the final value is exactly 10000.
// When run against the vulnerable version, the race condition typically causes the test to fail.
// When run against the fixed version employing atomic operations, the test passes.

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn verify_counter() {
        let final_value = run();
        // Assert that the counter matches the expected total.
        // For the vulnerable version this assertion is expected to fail intermittently.
        assert_eq!(final_value, 10000, "Final counter value does not match expected value.");
    }
}
