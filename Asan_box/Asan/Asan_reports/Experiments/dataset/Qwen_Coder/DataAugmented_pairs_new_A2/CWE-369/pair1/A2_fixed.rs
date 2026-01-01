#![allow(dead_code)]
use std::thread;
trait Calculation {
    fn evaluate(&self, numerator: i32, denominator: i32) -> i32;
}
struct Processor;
impl Calculation for Processor {
    fn evaluate(&self, numerator: i32, denominator: i32) -> i32 {
        if denominator == 0 {
            0
        } else {
            numerator / denominator
        }
    }
}
fn execute(numerator: i32, denominator: i32) -> i32 {
    let processor = Processor;
    let handle = thread::spawn(move || {
        processor.evaluate(numerator, denominator)
    });
    handle.join().unwrap()
}
pub fn calculate(numerator: i32, denominator: i32) -> i32 {
    execute(numerator, denominator)
}
fn main() {
    let result = execute(10, 0);
    println!("Calculation result: {}", result);
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
