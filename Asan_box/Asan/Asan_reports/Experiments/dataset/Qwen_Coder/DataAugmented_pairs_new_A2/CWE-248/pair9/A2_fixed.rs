use std::sync::Arc;
use std::thread;
use std::panic;
pub struct Controller;
impl Controller {
    pub fn run(&self, input: i32) -> Result<i32, &'static str> {
        let shared = Arc::new(input);
        let cloned = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            let res = panic::catch_unwind(|| {
                if *cloned == 0 {
                    Err("Division by zero")
                } else {
                    Ok(100 / *cloned)
                }
            });
            match res {
                Ok(inner) => inner,
                Err(_) => Err("Thread panicked")
            }
        });
        match handle.join() {
            Ok(inner) => inner,
            Err(_) => Err("Thread join failed")
        }
    }
}
fn main() {
    let controller = Controller;
    match controller.run(0) {
        Ok(value) => println!("Result: {}", value),
        Err(err) => println!("Handled error: {}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::Controller;
    use std::panic;

    // This test function serves as an oracle. It calls the public run()
    // method with a boundary input (0) that would trigger a division-by-zero.
    // In the corrected version, the error is caught and an Err result is returned.
    // In the vulnerable version, a panic occurs, which is caught by the outer 
    // catch_unwind, causing the test to fail.
    #[test]
    fn test_catching_behavior() {
        let controller = Controller;
        let result = panic::catch_unwind(|| controller.run(0));
        match result {
            Ok(inner) => {
                // For the fixed version, we expect an error result.
                assert!(inner.is_err(), "Expected an error condition when processing 0.");
            },
            Err(_) => {
                // The vulnerable version causes an uncaught panic.
                panic!("Test failure: Uncaught panic occurred during execution.");
            }
        }
    }
}
