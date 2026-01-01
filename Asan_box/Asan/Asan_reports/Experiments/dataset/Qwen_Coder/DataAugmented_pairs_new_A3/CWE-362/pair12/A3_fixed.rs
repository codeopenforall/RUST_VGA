use std::sync::atomic::{AtomicU32, Ordering};
static GLOBAL: AtomicU32 = AtomicU32::new(0);

pub fn run() -> u32 {
    for _ in 0..10_000 {
        GLOBAL.fetch_add(1, Ordering::SeqCst);
    }
    GLOBAL.load(Ordering::SeqCst)
}

////////////////////////////////////////////////////////////
// Test Oracle Function for Race Condition Vulnerability
// This test asserts that the final counter value equals 10,000.
// With the corrected implementation, the test will pass, whereas 
// the vulnerable implementation is highly likely to produce a 
// value less than the expected total due to lost updates.
////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    // Import `run` from the current module. When linking with the vulnerable version,
    // the unsynchronized run() would likely yield an incorrect counter value.
    use super::run;

    #[test]
    fn test_counter_integrity() {
        // Run the computation.
        let result = run();
        // The expected final counter value after 10*1000 increments is 10,000.
        // The vulnerable version may fail this assertion.
        assert_eq!(result, 10_000, "Final counter value should be 10000, indicating proper atomic updates.");
    }
}
