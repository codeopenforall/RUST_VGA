use std::sync::{Arc, Mutex};
use std::thread;

struct BufferData {
    data: Vec<u8>,
}

impl BufferData {
    fn initialize(size: usize) -> Self {
        let mut vec = Vec::with_capacity(size);
        for i in 0..(size - 1) {
            vec.push(i as u8);
        }
        unsafe {
            vec.set_len(size);
        }
        BufferData { data: vec }
    }

    fn calculate_sum(&self) -> u64 {
        self.data.iter().map(|&x| x as u64).sum()
    }
}

fn main() {
    let shared_data = Arc::new(Mutex::new(BufferData::initialize(10)));
    let mut thread_handles = vec![];

    for _ in 0..4 {
        let cloned_data = shared_data.clone();
        thread_handles.push(thread::spawn(move || {
            let locked_data = cloned_data.lock().unwrap();
            println!("Sum: {}", locked_data.calculate_sum());
        }));
    }

    for handle in thread_handles {
        handle.join().unwrap();
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
