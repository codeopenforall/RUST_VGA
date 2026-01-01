use std::sync::{Arc, Mutex};
use std::thread;
use std::env;

trait Operation {
    fn compute(&self, divisor: i32) -> i32;
}

struct Engine;

impl Operation for Engine {
    fn compute(&self, divisor: i32) -> i32 {
        if divisor == 0 {
            0
        } else {
            let numerator = 10;
            numerator / divisor
        }
    }
}

fn perform_calculation(divisor: i32) -> i32 {
    let engine = Engine;
    engine.compute(divisor)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        args[1].parse::<i32>().unwrap_or(1)
    } else {
        1
    };
    let shared_input = Arc::new(Mutex::new(input));
    let shared_calculation = Arc::new(Mutex::new(perform_calculation));
    let thread_input = Arc::clone(&shared_input);
    let thread_calc = Arc::clone(&shared_calculation);
    let handle = thread::spawn(move || {
        let val = *thread_input.lock().unwrap();
        let calc = thread_calc.lock().unwrap();
        let result = calc(val);
        println!("Result: {}", result);
    });
    handle.join().unwrap();
}

#[cfg(test)]
mod tests {
    use std::panic;
    
    // This test oracle calls the shared perform_calculation function.
    // It provides an input of 0 which should trigger a panic (in the vulnerable version)
    // or safely return 0 (in the corrected version).
    #[test]
    fn test_divide_by_zero_handling() {
        let result = panic::catch_unwind(|| {
            // This function is expected to be imported from the respective module.
            super::perform_calculation(0)
        });
        match result {
            Ok(value) => {
                // In the correct version, a zero divisor returns 0.
                assert_eq!(value, 0, "Expected safe handling to return 0 for zero divisor");
            },
            Err(_) => {
                // The vulnerable version would panic on a zero divisor.
                panic!("Unsafe division by zero occurred");
            }
        }
    }
}
