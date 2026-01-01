struct Calculator;

impl Calculator {
    pub fn execute(&self, denominator: i32) -> Result<i32, &'static str> {
        if denominator == 0 {
            Err("Division by zero encountered")
        } else {
            Ok(100 / denominator)
        }
    }
}

fn main() {
    let calc = Calculator;
    let thread_val = 0; // Example value, can be changed as needed

    match calc.execute(thread_val) {
        Ok(result) => println!("Output: {}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

////////////////////////////////////////////////////////////
// Test Oracle: Ensures that the zero denominator case
// is handled properly.
//
// This test calls the execute() function with a zero
// denominator. For the vulnerable version, the unsafe
// division will panic, causing the test to fail.
// For the fixed version, the function should return an
// error result, thereby making the test pass.
////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn oracle() {
        let calc = Calculator;
        // Execute with a zero divisor.
        let res = panic::catch_unwind(|| {
            calc.execute(0)
        });
        // The fixed version should not panic.
        assert!(res.is_ok(), "Function should not panic on zero input");
        let result = res.unwrap();
        // For a zero denominator, the fixed version returns an error.
        assert!(result.is_err(), "Fixed version should return an error on division by zero");
    }
}
