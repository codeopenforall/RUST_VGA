use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::thread;
pub fn run() -> u32 {
    let counter = Arc::new(AtomicU32::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            }
        }));
    }
    for handle in handles {
        let _ = handle.join();
    }
    counter.load(Ordering::SeqCst)
}
fn main() {
    let final_count = run();
    println!("Final counter value: {}", final_count);
}

//////////////////////////////////////////////////////////////
// The test oracle verifies that the concurrent increment routine works as expected.
// It asserts that the final counter value is exactly 10000. For the vulnerable version,
// the unsynchronized updates are likely to yield an incorrect counter value, failing the test.
// For the fixed version, the test passes reliably.
//////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    // Import the run function from the corresponding module.
    // When testing the vulnerable version, this test should frequently fail,
    // while it passes consistently for the fixed version.
    use super::run;

    #[test]
    fn counter_oracle() {
        let result = run();
        // Expected final value after 10 threads * 1000 increments.
        assert_eq!(result, 10000, "Final counter did not reach the expected 10000 due to a race condition");
    }
}
