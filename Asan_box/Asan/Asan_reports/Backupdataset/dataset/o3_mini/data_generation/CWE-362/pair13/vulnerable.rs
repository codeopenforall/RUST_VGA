use std::cell::UnsafeCell;
use std::thread;
static mut GLOBAL: UnsafeCell<u32> = UnsafeCell::new(0);
fn update(count: u32) {
    for _ in 0..count {
        unsafe {
            let ptr = GLOBAL.get();
            let current = *ptr;
            thread::yield_now();
            *ptr = current.wrapping_add(1);
        }
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
    unsafe { *GLOBAL.get() }
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
