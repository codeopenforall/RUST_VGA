use std::sync::{Arc, Mutex};
use std::thread;

struct Manager {
    amount: i32,
}

impl Manager {
    unsafe fn unsafe_modification(&mut self, increment: i32) -> i32 {
        if self.amount == 0 {
            1 
        } else {
            self.amount += increment;
            0 
        }
    }
    fn modify(&mut self, increment: i32) -> Result<(), &'static str> {
        unsafe {
            let res = self.unsafe_modification(increment);
            if res == 0 {
                Ok(())
            } else {
                Err("Modification failed")
            }
        }
    }
}

fn simulate(start: i32, increment: i32) -> Result<i32, &'static str> {
    let mut m = Manager { amount: start };
    m.modify(increment)?;
    Ok(m.amount)
}

fn main() {
    let manager = Arc::new(Mutex::new(Manager { amount: 0 }));
    let manager_clone = Arc::clone(&manager);
    let handle = thread::spawn(move || {
        let mut m = manager_clone.lock().unwrap();
        m.modify(10).unwrap();
    });
    handle.join().unwrap();
    println!("Amount: {}", manager.lock().unwrap().amount);
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
