use std::sync::{Arc, Mutex};
use std::thread;

struct Data {
    counter: u32,
}

impl Data {
    fn new(count: u32) -> Self {
        Data { counter: count }
    }

    fn subtract_val(&mut self, val: i32) {
        if val > 0 {
            self.counter = self.counter.saturating_sub(val as u32);
        }
    }
}

fn main() {
    let shared = Arc::new(Mutex::new(Data::new(0)));
    let handles: Vec<_> = (0..1)
        .map(|_| {
            let shared_clone = Arc::clone(&shared);
            thread::spawn(move || {
                let mut data = shared_clone.lock().unwrap();
                data.subtract_val(-1);
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    let data = shared.lock().unwrap();
    println!("Final counter value: {}", data.counter);
}

//////////////////////////////////////////////////////////////
// Test Oracle: The following unit test verifies the underflow protection.
// It creates an instance with an initial counter of 0, calls the subtraction 
// method with -1, and asserts that the counter remains 0.
// In the vulnerable code, the negative value is cast to a large u32 causing the
// counter to become 4294967295, which fails the test.
//////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_underflow_protection() {
        let mut data = Data::new(0);
        data.subtract_val(-1);
        // For the fixed version the subtraction with a negative value is skipped.
        // The test asserts that the counter remains 0.
        assert_eq!(data.counter, 0, "Integer underflow vulnerability detected!");
    }
}
