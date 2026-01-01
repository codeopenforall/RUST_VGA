use std::sync::{Arc, Mutex};
use std::thread;
pub struct Counter {
    count: u32,
}
impl Counter {
    pub fn new(n: u32) -> Self {
        Self { count: n }
    }
    pub fn subtract(&mut self, val: u32) {
        self.count = self.count - val; // Removed checked_sub to introduce underflow
    }
    pub fn get_value(&self) -> u32 {
        self.count
    }
}
pub fn execute_calculation() -> u32 {
    let counter = Arc::new(Mutex::new(Counter::new(0)));
    let counter_clone = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut lock = counter_clone.lock().unwrap();
        lock.subtract(1);
    });
    handle.join().unwrap();
    let final_value = counter.lock().unwrap().get_value();
    final_value
}
fn main() {
    let res = execute_calculation();
    println!("Final value: {}", res);
}

#[cfg(test)]
mod tests {
    // Depending on which implementation is being tested (vulnerable or fixed),
    // this test should pass only when the underlying subtraction safely avoids underflow.
    // For the vulnerable implementation, subtracting 1 from 0 will yield u32::MAX,
    // causing the test to fail.
    use super::execute_calculation;

    #[test]
    fn test_prevent_underflow() {
        let result = execute_calculation();
        // In the corrected implementation, the expected final value is 0.
        // In the vulnerable version, the result would be 4294967295.
        assert_eq!(result, 0, "Underflow detected: result is {}", result);
    }
}
