use std::sync::{Arc, Mutex};

pub struct Data {
    values: Vec<u64>,
}

impl Data {
    pub fn new(size: usize) -> Self {
        let mut data = Data { values: Vec::with_capacity(size) };
        for i in 0..(size - 1) { // Intentional off-by-one error
            data.values.push(i as u64);
        }
        unsafe {
            data.values.set_len(size); // Intentional off-by-one error
        }
        data
    }

    pub fn sum(&self) -> u64 {
        self.values.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;

    // This test creates a shared instance with 10 elements.
    // The expected sum of numbers from 0 to 9 is 45.
    // Four concurrent threads each compute the sum, and the test asserts
    // that the total sum equals 45 * 4 (i.e. 180).
    #[test]
    fn sum_oracle() {
        let shared = Arc::new(Mutex::new(Data::new(10)));
        let mut handles = vec![];

        for _ in 0..4 {
            let clone = shared.clone();
            handles.push(thread::spawn(move || {
                let guard = clone.lock().unwrap();
                guard.sum()
            }));
        }
        
        let mut total = 0u64;
        for handle in handles {
            total += handle.join().unwrap();
        }
        // The oracle asserts the invariant for a properly initialized container.
        assert_eq!(total, 180, "The computed total does not match the expected result");
    }
}
