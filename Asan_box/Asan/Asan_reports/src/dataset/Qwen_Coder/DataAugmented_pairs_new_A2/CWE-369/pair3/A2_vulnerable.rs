use std::sync::{Arc, Mutex};
use std::thread;

struct Arithmetic;

impl Arithmetic {
    fn compute(numerator: i32, divisor: i32) -> Option<i32> {
        if divisor == 0 {
            return None;
        }
        unsafe {
            let ptr: *const i32 = &divisor;
            let val = *ptr;
            Some(numerator / val)
        }
    }
}

fn main() {
    let data = Arc::new(Mutex::new(vec![(100, 5), (50, 0), (30, 3)]));
    let mut handles = vec![];
    for _ in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let tasks = data_clone.lock().unwrap();
            for &(num, den) in tasks.iter() {
                let res = Arithmetic::compute(num, den);
                match res {
                    Some(val) => println!("Result: {}", val),
                    None => println!("Unexpected None returned"),
                }
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        let _ = handle.join();
    }
}

////////////////////////////////////////////
// Test Oracle
////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn division_zero_test() {
        // The function should safely handle a zero divisor by returning None without panicking.
        let result = std::panic::catch_unwind(|| {
            Arithmetic::compute(50, 0)
        });
        // For the fixed version, the operation should not panic.
        assert!(
            result.is_ok(),
            "The arithmetic operation panicked on division by zero"
        );
        let outcome = result.unwrap();
        // The expected behavior is to return None when a zero divisor is provided.
        assert!(
            outcome.is_none(),
            "Expected None when dividing by zero, but got a value"
        );
    }
}
