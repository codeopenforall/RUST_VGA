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