use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

trait Operation {
    fn compute(&self, dividend: i32, divisor: i32) -> i32;
}

struct Calculator;

impl Operation for Calculator {
    fn compute(&self, dividend: i32, divisor: i32) -> i32 {
        unsafe {
            dividend / divisor
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <divisor>", args[0]);
        return;
    }
    let divisor: i32 = args[1].parse().unwrap_or(1);
    let calc = Arc::new(Calculator);
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    for i in 0..5 {
        let calc_clone = Arc::clone(&calc);
        let results_clone = Arc::clone(&results);
        let thread_divisor = divisor;
        handles.push(thread::spawn(move || {
            let res = calc_clone.compute(100, thread_divisor);
            let mut vec = results_clone.lock().unwrap();
            vec.push(res + i);
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let final_results = results.lock().unwrap();
    let sum: i32 = final_results.iter().sum();
    println!("Final computed sum: {}", sum);
}

/*
This test oracle is designed to validate the behavior of the calculator implementation.
It directly instantiates the Calculator and tests two cases:
  1. A safe division (e.g., 100 divided by 10) which should return 10.
  2. A division with a zero divisor:
       - For the corrected implementation, the function should return 0.
       - For the vulnerable implementation, the unsafe division will trigger a panic.
The test uses catch_unwind to capture panics. For the vulnerable implementation, the test is expected
to fail, while the fixed implementation should pass.
*/
#[cfg(test)]
mod tests {
    use std::panic;
    // Import the traits and structs from the code under test.
    use super::{Calculator, Operation};

    #[test]
    fn test_division_behavior() {
        let calc = Calculator;

        // Test a normal division operation.
        let safe_result = calc.compute(100, 10);
        assert_eq!(safe_result, 10, "Expected 100 / 10 to equal 10");

        // Test division by zero.
        let zero_result = panic::catch_unwind(|| {
            calc.compute(100, 0)
        });

        match zero_result {
            // If the computation does not panic, expect a safe value of 0.
            Ok(val) => assert_eq!(val, 0, "For a zero divisor, the result should be 0"),
            // For the vulnerable implementation, a panic is expected.
            Err(_) => panic!("The computation panicked on division by zero"),
        }
    }
}
