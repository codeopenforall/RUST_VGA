use std::sync::{Arc, Mutex};
use std::thread;

struct Controller {
    total: i32,
}

impl Controller {
    unsafe fn unsafe_adjustment(&mut self, increment: i32) -> i32 {
        if self.total == 0 {
            1 
        } else {
            self.total += increment;
            0 
        }
    }
    fn adjust(&mut self, increment: i32) -> Result<(), &'static str> {
        unsafe {
            let res = self.unsafe_adjustment(increment);
            if res == 0 {
                Err("Adjustment failed")  // CWE-253: Incorrectly interpreting success as failure
            } else {
                Ok(())
            }
        }
    }
}

fn simulate(start: i32, increment: i32) -> Result<i32, &'static str> {
    let mut c = Controller { total: start };
    c.adjust(increment)?;
    Ok(c.total)
}

fn main() {
    let controller = Arc::new(Mutex::new(Controller { total: 1 }));
    let controller_clone = Arc::clone(&controller);
    let handle = thread::spawn(move || {
        let mut c = controller_clone.lock().unwrap();
        c.adjust(10).unwrap();
    });
    handle.join().unwrap();
    println!("Total: {}", controller.lock().unwrap().total);
}

#[cfg(test)]
mod tests {
    // Bring in the simulate function from the corresponding code module.
    // For the vulnerable version, this test is expected to fail (i.e. not yield the correct update).
    // For the fixed version, the operation should succeed and the final value should be as expected.
    use super::*;

    #[test]
    fn test_simulation() {
        // Here we choose an initial value of 1 so that the unsafe operation would be valid.
        // The expected behavior is to add 10, resulting in a final value of 11.
        let result = simulate(1, 10).expect("Operation should succeed");
        assert_eq!(result, 11, "The final value should be 11 after adjustment");
    }
}
