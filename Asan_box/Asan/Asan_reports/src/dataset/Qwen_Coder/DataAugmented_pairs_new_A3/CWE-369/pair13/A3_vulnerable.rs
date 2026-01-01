use std::sync::{Arc, Mutex};
use std::thread;

struct Data {
    numerator: i32,
}

trait Calculation {
    fn compute(&self, divisor: i32) -> i32;
}

impl Calculation for Data {
    fn compute(&self, divisor: i32) -> i32 {
        unsafe {
            self.numerator / divisor
        }
    }
}

fn perform(divisor: i32) -> Result<i32, &'static str> {
    let data = Arc::new(Data { numerator: 100 });
    let result = Arc::new(Mutex::new(None));
    let data_ref = Arc::clone(&data);
    let res_ref = Arc::clone(&result);
    let handle = thread::spawn(move || {
        let res = data_ref.compute(divisor);
        let mut lock = res_ref.lock().unwrap();
        *lock = Some(res);
    });
    handle.join().unwrap();
    let final_res = result.lock().unwrap();
    match *final_res {
        Some(v) => Ok(v),
        None => Err("No result computed"),
    }
}

fn main() {
    let user_input = 0;
    match perform(user_input) {
        Ok(val) => println!("Result: {}", val),
        Err(e) => eprintln!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn division_behavior() {
        // Test with a valid divisor; both versions should return correct result.
        let result = perform(5);
        assert_eq!(result.unwrap(), 20);

        // Test with a zero divisor.
        // In the fixed version, perform(0) returns an Err; in the vulnerable version, it triggers a panic.
        let result_zero = panic::catch_unwind(|| {
            perform(0)
        });
        match result_zero {
            Ok(res) => {
                // In fixed code, we expect an error message.
                assert!(res.is_err());
                assert_eq!(res.err().unwrap(), "Attempted division by zero");
            },
            Err(_) => {
                // In the vulnerable code, the panic indicates the vulnerability.
                panic!("Test failed: Division by zero caused a panic.");
            }
        }
    }
}
