trait Arithmetic {
    fn operate(&self, numerator: i32, denominator: i32) -> i32;
}

struct Engine;

impl Arithmetic for Engine {
    fn operate(&self, numerator: i32, denominator: i32) -> i32 {
        unsafe {
            let ptr = &denominator as *const i32;
            let div = *ptr;
            numerator / div
        }
    }
}

fn process(numerator: i32, denominator: i32) -> i32 {
    let engine = Engine;
    engine.operate(numerator, denominator)
}

pub fn calculate(numerator: i32, denominator: i32) -> i32 {
    process(numerator, denominator)
}

fn main() {
    let result = calculate(10, 0);
    println!("Result: {}", result);
}

//////////////////////////////////////////////
// Test Oracle Function
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::panic;
    // Import the public API 'calculate' from the current crate.
    use crate::calculate;

    #[test]
    fn oracle_test() {
        // When invoking calculate(10, 0):
        // - The vulnerable version will panic (caught as an error).
        // - The fixed version will return 0.
        let result = panic::catch_unwind(|| {
            calculate(10, 0)
        });
        // Assert that the result is 0.
        // In the vulnerable code, since a panic occurs, this assertion will fail.
        match result {
            Ok(value) => assert_eq!(value, 0, "Expected result 0 for safe division."),
            Err(_) => panic!("The operation panicked due to a divide-by-zero error."),
        }
    }
}
