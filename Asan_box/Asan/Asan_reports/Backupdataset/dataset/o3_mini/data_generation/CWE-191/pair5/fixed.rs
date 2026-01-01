use std::sync::{Arc, Mutex};
use std::thread;
struct Accumulator {
    total: u32,
}
impl Accumulator {
    fn new(initial: u32) -> Self {
        Accumulator { total: initial }
    }
    fn update(&mut self, amount: u32) {
        self.total = self.total.checked_sub(amount).unwrap_or(0);
    }
    fn value(&self) -> u32 {
        self.total
    }
}
fn main() {
    let data = Arc::new(Mutex::new(Accumulator::new(0)));
    let mut handles = vec![];
    for _ in 0..4 {
        let data_cloned = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut acc = data_cloned.lock().unwrap();
            acc.update(1);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let acc = data.lock().unwrap();
    println!("Result: {}", acc.value());
}


//////////////////////////////
// Test Oracle Function
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subtraction_behavior() {
        // Create a new accumulator starting at 0.
        let mut accumulator = Accumulator::new(0);
        // Attempt to subtract 1.
        accumulator.update(1);
        // For the fixed code, subtracting from 0 should yield 0.
        // In the vulnerable code, the result would underflow (e.g., 4294967295 for u32).
        assert_eq!(accumulator.value(), 0, "Underflow detected: the value is not safe");
    }
}
