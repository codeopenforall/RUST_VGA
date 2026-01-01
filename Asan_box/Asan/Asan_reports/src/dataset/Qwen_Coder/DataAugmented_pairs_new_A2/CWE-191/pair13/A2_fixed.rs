use std::sync::{Arc, Mutex, Barrier};
use std::thread;

pub struct Container {
    pub value: u32,
}

impl Container {
    pub fn lessen(&mut self, amount: u32) {
        self.value = self.value.saturating_sub(amount);
    }
}

pub fn execute(amount: u32) -> u32 {
    let data = Arc::new(Mutex::new(Container { value: 10 }));
    let barrier = Arc::new(Barrier::new(2));
    let data_clone = Arc::clone(&data);
    let barrier_clone = Arc::clone(&barrier);
    let handle = thread::spawn(move || {
        barrier_clone.wait();
        let mut guard = data_clone.lock().unwrap();
        guard.lessen(amount);
    });
    barrier.wait();
    handle.join().unwrap();
    let guard = data.lock().unwrap();
    guard.value
}

fn main() {
    let result = execute(20);
    println!("Final value: {}", result);
}

#[cfg(test)]
mod tests {
    use super::execute;
    
    // This test oracle expects that subtracting 20 from 10 will yield 0.
    // For the vulnerable code, the result will be a large number (e.g., 4294967286), causing the test to fail.
    #[test]
    fn test_underflow_prevention() {
        let result = execute(20);
        assert_eq!(result, 0, "Expected result is 0 after safe subtraction; vulnerability persists if not fixed.");
    }
}
