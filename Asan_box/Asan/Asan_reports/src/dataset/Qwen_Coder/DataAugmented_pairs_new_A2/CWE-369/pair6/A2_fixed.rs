use std::io;
use std::thread;
use std::sync::mpsc;

trait Operations {
    unsafe fn div_safe(&self, numerator: i32, denominator_ptr: *const i32) -> Result<i32, &'static str>;
}

struct Arithmetic;

impl Operations for Arithmetic {
    unsafe fn div_safe(&self, numerator: i32, denominator_ptr: *const i32) -> Result<i32, &'static str> {
        let denominator = *denominator_ptr;
        if denominator == 0 {
            return Err("Division by zero");
        }
        Ok(numerator / denominator)
    }
}

fn main() {
    let boxed = Box::new(0i32);
    let mut raw_ptr: *mut i32 = Box::into_raw(boxed);
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let value: i32 = input.trim().parse().expect("Invalid integer input");
        tx.send(value).expect("Channel send failed");
    });
    let value = rx.recv().expect("Channel receive failed");
    unsafe {
        *raw_ptr = value;
    }
    let calc = Arithmetic;
    let result = unsafe { calc.div_safe(100, raw_ptr as *const i32) };
    match result {
        Ok(res) => println!("Result: {}", res),
        Err(e) => println!("Error: {}", e),
    }
    unsafe {
        Box::from_raw(raw_ptr);
    }
}

#[cfg(test)]
mod tests {
    use std::panic;
    // Bring in the trait and struct from the implementation.
    use super::{Arithmetic, Operations};

    // The test oracle uses a fixed input (zero) to verify behavior.
    // For the vulnerable version, an unchecked division by zero would result in a panic.
    // For the corrected version, the safe check will return an Err.
    #[test]
    fn test_division_by_zero() {
        // Prepare a boxed integer initialized to zero.
        let boxed = Box::new(0i32);
        let raw_ptr = Box::into_raw(boxed);

        let calc = Arithmetic;
        // Use panic::catch_unwind to capture panics in the vulnerable version.
        let result = panic::catch_unwind(|| unsafe {
            // Calling the division method with denominator zero.
            calc.div_safe(100, raw_ptr as *const i32)
        });

        // Clean up memory.
        unsafe {
            Box::from_raw(raw_ptr);
        }

        // The test expectation:
        // - In the vulnerable version, division by zero panics and result.is_err() is true.
        // - In the fixed version, the method handles the zero divisor and returns Err("Division by zero").
        // This oracle asserts that the fixed version does not panic and returns an error variant.
        // Adjust the assertions based on the intended behavior:
        if result.is_err() {
            // Vulnerable behavior: a panic occurred.
            panic!("Test failed: division by zero caused a panic");
        } else {
            // Expecting a safe error return.
            if let Ok(inner) = result {
                assert!(inner.is_err(), "Expected an error due to division by zero");
                assert_eq!(inner.unwrap_err(), "Division by zero");
            }
        }
    }
}
