use std::sync::Arc;
use std::thread;

struct Calculation {
    top: i32,
    bottom: i32,
}

impl Calculation {
    fn perform(&self) -> i32 {
        unsafe {
            let ptr = &self.top as *const i32;
            let value = *ptr;
            value / self.bottom
        }
    }
}

fn main() {
    let calc = Arc::new(Calculation { top: 100, bottom: 0 });
    let calc_clone = Arc::clone(&calc);
    let handle = thread::spawn(move || {
        calc_clone.perform()
    });
    let outcome = handle.join().expect("Thread panicked due to divide-by-zero");
    println!("Outcome: {}", outcome);
}

////////////////////////////////////////////////////////
// Test oracle function for verifying the divide-by-zero fix.
//
// The oracle calls the main functionality of the compiled program 
// (by invoking the main function within a catch_unwind context).
// For the vulnerable variant, the execution will panic due to an unchecked
// division by zero, causing the test to fail.
// For the corrected version, the execution handles the zero divisor gracefully
// by returning an error, and the main function completes without a panic.
////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::panic;

    // The test harness assumes that the main function defined in the binary
    // encapsulates the core arithmetic operation.
    #[test]
    fn test_divide_by_zero_handling() {
        let result = panic::catch_unwind(|| {
            // Calling main to simulate program execution.
            // Note: The main function should be defined in the binary (outside the test module).
            crate::main();
        });
        // For the corrected version, main() must complete without panic.
        assert!(result.is_ok(), "Main function panicked, division by zero vulnerability remains active");
    }
}
