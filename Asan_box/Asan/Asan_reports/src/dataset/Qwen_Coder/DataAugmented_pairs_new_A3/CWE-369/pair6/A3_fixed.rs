use std::fmt;

// Define the Operations trait with a div_safe method.
pub trait Operations {
    fn div_safe(&self, numerator: i32, denominator_ptr: *const i32) -> Result<i32, &'static str>;
}

// Implement the Operations trait for a struct named Arithmetic.
pub struct Arithmetic;

impl Operations for Arithmetic {
    fn div_safe(&self, numerator: i32, denominator_ptr: *const i32) -> Result<i32, &'static str> {
        // Dereference the pointer to get the denominator value.
        let denominator = unsafe { *denominator_ptr };
        
        // Check if the denominator is zero to prevent division by zero.
        if denominator == 0 {
            return Err("Division by zero");
        }
        
        // Perform the division if the denominator is not zero.
        Ok(numerator / denominator)
    }
}

// Implement the Display trait for the Arithmetic struct for easy printing.
impl fmt::Display for Arithmetic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Arithmetic")
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
