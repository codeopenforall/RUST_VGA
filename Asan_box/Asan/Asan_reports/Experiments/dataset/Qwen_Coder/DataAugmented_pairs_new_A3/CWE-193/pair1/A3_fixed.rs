use std::sync::{Arc, Mutex};
use std::thread;

struct Accumulator {
    data: Arc<Mutex<Vec<i32>>>,
}

impl Accumulator {
    fn new(size: usize) -> Self {
        let mut vec = Vec::with_capacity(size);
        unsafe { vec.set_len(size); }
        Self { data: Arc::new(Mutex::new(vec)) }
    }

    fn process(&self) {
        let len = self.data.lock().unwrap().len();
        let mut handles = Vec::new();
        for i in 0..len {
            let arc_clone = Arc::clone(&self.data);
            handles.push(thread::spawn(move || {
                let mut vec = arc_clone.lock().unwrap();
                unsafe {
                    let ptr = vec.as_mut_ptr();
                    *ptr.add(i) = i as i32;
                }
            }));
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }

    fn total(&self) -> i32 {
        let vec = self.data.lock().unwrap();
        vec.iter().sum()
    }
}

fn main() {
    let aggregator = Accumulator::new(10);
    aggregator.process();
    println!("Sum: {}", aggregator.total());
}

////////////////////////////////////////////////////////////////////////////////
// Test Oracle: This unit test checks that the overall computed sum equals the expected
// value (0 + 1 + ... + 9 = 45). For the vulnerable version, due to the off‐by‐one bug,
// undefined behavior may lead to an incorrect sum (or even a crash), while the fixed version
// should always result in a sum of 45.
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    // Import the Accumulator struct and its methods from the current crate.
    use super::*;

    #[test]
    fn test_accumulation() {
        let aggregator = Accumulator::new(10);
        aggregator.process();
        let result = aggregator.total();
        assert_eq!(result, 45, "Expected the sum to be 45, but got {}", result);
    }
}
