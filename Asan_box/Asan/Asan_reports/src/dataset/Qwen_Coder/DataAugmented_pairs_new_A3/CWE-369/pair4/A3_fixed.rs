pub struct Calculator;

impl Calculator {
    pub unsafe fn compute(&self, a: i32, b: i32) -> Result<i32, &'static str> {
        if b == 0 {
            return Err("Division by zero");
        }
        let divisor = b; // Assuming b_ptr was a pointer to b, which is unnecessary here
        Ok(a / divisor)
    }
}

pub unsafe fn perform_division(a: i32, b: i32) -> i32 {
    let calc = Calculator;
    match calc.compute(a, b) {
        Ok(val) => val,
        Err(err) => {
            println!("Error encountered: {}", err);
            0
        }
    }
}

//////////////////////////////////////////
// Test Oracle Function
//////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::panic;

    // This test oracle targets the perform_division function.
    // When given a divisor of zero, the vulnerable version will panic,
    // whereas the corrected version returns a safe fallback value (0).
    #[test]
    fn division_oracle() {
        // The perform_division function must be in scope from the binary under test.
        // For demonstration purposes, we assume it is linked from the current crate.
        // Replace `crate::perform_division` with the correct path if necessary.
        let result = panic::catch_unwind(|| unsafe { crate::perform_division(10, 0) });
        
        // In the fixed version, the result should be Ok(0),
        // while in the vulnerable version, a panic will be caught.
        match result {
            Ok(val) => assert_eq!(val, 0, "Expected fallback value of 0 when dividing by zero"),
            Err(_) => panic!("Test failed: Division by zero caused a panic, indicating a vulnerability."),
        }
    }
}
