use std::thread;
use std::panic;
struct Worker;
impl Worker {
    fn process(&self, input: i32) -> Option<i32> {
        let handle = thread::spawn(move || {
            panic::catch_unwind(|| unsafe { Self::unsafe_op(input) })
                .unwrap_or(-1)
        });
        Some(handle.join().unwrap())
    }
    unsafe fn unsafe_op(input: i32) -> i32 {
        let boxed = Box::new(input);
        let raw_ptr = Box::into_raw(boxed);
        let result;
        if *raw_ptr == 0 {
            result = -1;
        } else {
            result = *raw_ptr;
        }
        drop(Box::from_raw(raw_ptr));
        result
    }
}
fn main() {
    let worker = Worker;
    let res = worker.process(0);
    match res {
        Some(val) if val == -1 => println!("Handled error gracefully"),
        Some(val) => println!("Result: {}", val),
        None => println!("No result obtained"),
    }
}

/////////////////// Test Oracle ////////////////////////
// This test oracle is designed to verify that processing a boundary value (0)
// is safely handled in the fixed version, while the vulnerable version will panic.
//
// The contract: When invoking process(0), the fixed version should return Some(-1),
// representing a handled error. In the vulnerable version, the thread will panic,
// causing the test to fail.
//
// Note: Do not include this oracle code in the vulnerable or fixed binaries.
#[cfg(test)]
mod tests {
    use std::panic;
    use super::Worker;

    #[test]
    fn test_handling() {
        let worker = Worker;
        // Attempt to process a boundary value that historically triggered a panic.
        let result = panic::catch_unwind(|| worker.process(0));
        match result {
            Ok(outcome) => {
                // In the fixed version, outcome should be Some(-1),
                // indicating that the error was handled gracefully.
                assert_eq!(outcome, Some(-1), "Expected error code -1, but got {:?}", outcome);
            },
            Err(_) => {
                // If a panic is caught, the vulnerability is still present.
                panic!("Abnormal termination: the boundary value caused an uncaught panic");
            },
        }
    }
}
