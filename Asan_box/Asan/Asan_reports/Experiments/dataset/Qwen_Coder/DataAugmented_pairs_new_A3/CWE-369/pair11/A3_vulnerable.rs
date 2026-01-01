use std::sync::mpsc;
use std::thread;
use std::panic;

struct Module;
trait Operation {
    unsafe fn transform(&self, a: i32, b: i32) -> i32;
}
impl Operation for Module {
    unsafe fn transform(&self, a: i32, b: i32) -> i32 {
        let ptr: *const i32 = &b;
        let factor = *ptr;
        a / factor
    }
}
fn compute(a: i32, b: i32) -> i32 {
    let m = Module;
    unsafe { m.transform(a, b) }
}
pub fn run_calc(a: i32, b: i32) -> Result<i32, &'static str> {
    let res = panic::catch_unwind(|| compute(a, b));
    match res {
        Ok(val) => Ok(val),
        Err(_) => Err("Division by zero occurred"), // Intentional defect: incorrect error message
    }
}
fn main_thread() {
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let r = compute(10, 0);
        tx.send(r).unwrap();
    });
    let _ = rx.recv().unwrap();
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
