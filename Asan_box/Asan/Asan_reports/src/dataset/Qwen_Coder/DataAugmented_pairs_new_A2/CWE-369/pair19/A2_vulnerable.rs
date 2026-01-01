use std::sync::Arc;
use std::thread;

struct Calculator;

impl Calculator {
    fn compute(&self, a: i32, b: i32) -> Result<i32, &'static str> {
        unsafe {
            let ptr: *const i32 = &b;
            let divisor = *ptr;
            if divisor == 0 {
                return Err("Division by zero");
            }
            Ok(a / divisor)
        }
    }
}

fn main() {
    let calc = Arc::new(Calculator);
    let calc_clone = Arc::clone(&calc);
    let join_handle = thread::spawn(move || {
        calc_clone.compute(100, 0)
    });
    let res = join_handle.join().expect("Thread panicked unexpectedly.");
    match res {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}

//////////////////////////////
// Test Oracle for Division-by-Zero Handling
//////////////////////////////
#[cfg(test)]
mod tests {
    use std::panic;
    use std::sync::Arc;
    use std::thread;
    // Import the Calculator from the current crate.
    use crate::Calculator;

    #[test]
    fn test_division_by_zero() {
        let calc = Arc::new(Calculator);
        let calc_clone = Arc::clone(&calc);
        // Spawn a new thread to encapsulate division.
        let join_handle = thread::spawn(move || {
            // Use catch_unwind to intercept panics in the vulnerable version.
            panic::catch_unwind(|| calc_clone.compute(100, 0))
        });
        let result = join_handle.join().expect("Thread join failed.");

        match result {
            // For the fixed version, compute() returns Ok(Err("Division by zero"))
            // For the vulnerable version, a panic is caught and result is Err(..), failing the test.
            Ok(inner) => {
                match inner {
                    Ok(val) => panic!("Unexpected successful division: {}", val),
                    Err(e) => assert_eq!(e, "Division by zero", "Expected a division-by-zero error"),
                }
            }
            Err(_) => {
                panic!("Division by zero triggered a panic in unsafe arithmetic.");
            }
        }
    }
}
