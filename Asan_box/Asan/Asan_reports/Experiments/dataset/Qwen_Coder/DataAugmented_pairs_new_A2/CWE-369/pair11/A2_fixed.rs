use std::sync::mpsc;
use std::thread;

struct Module;

trait Operation {
    unsafe fn transform(&self, a: i32, b: i32) -> Result<i32, &'static str>;
}

impl Operation for Module {
    unsafe fn transform(&self, a: i32, b: i32) -> Result<i32, &'static str> {
        if b == 0 {
            return Err("Division by zero error");
        }
        let ptr: *const i32 = &b;
        let factor = *ptr;
        Ok(a / factor)
    }
}

fn compute(a: i32, b: i32) -> Result<i32, &'static str> {
    let m = Module;
    unsafe { m.transform(a, b) }
}

pub fn run_calc(a: i32, b: i32) -> Result<i32, &'static str> {
    compute(a, b)
}

fn main_thread() {
    let (tx, rx) = std::sync::mpsc::channel();
    let handle = thread::spawn(move || {
        let r = compute(10, 0);
        tx.send(r).unwrap();
    });
    let result = rx.recv().unwrap();
    match result {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {}", e)
    }
    handle.join().unwrap();
}

fn main() {
    main_thread();
}

#[cfg(test)]
mod tests {
    use std::panic;

    // The test oracle calls the public API run_calc with inputs that lead to division by zero.
    // For the fixed version, run_calc is expected to return an Err with the message "Division by zero error".
    // For the vulnerable version, the underlying unsafe computation will panic (even though run_calc catches
    // the panic internally, its behavior diverges from the fixed contract). This test ensures that the correct
    // error case is handled without a panic.
    #[test]
    fn test_division_by_zero() {
        // The catch_unwind here ensures that if an unexpected panic occurs, the test will fail.
        let outcome = panic::catch_unwind(|| {
            crate::run_calc(10, 0)
        });
        match outcome {
            Err(_) => panic!("Unexpected panic occurred, indicating a vulnerability."),
            Ok(result) => {
                match result {
                    Ok(_) => panic!("Expected an error due to division by zero, but got a successful result."),
                    Err(msg) => assert_eq!(msg, "Division by zero error"),
                }
            }
        }
    }
}
