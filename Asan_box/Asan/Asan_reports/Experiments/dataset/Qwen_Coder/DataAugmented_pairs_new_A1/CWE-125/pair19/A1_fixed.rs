use std::sync::{Arc, Mutex};
use std::thread;

trait Compute {
    fn compute(&self) -> u32;
}

struct DataHolder {
    data: Vec<u32>,
}

impl Compute for DataHolder {
    fn compute(&self) -> u32 {
        let len = self.data.len();
        let mut total: u32 = 0;
        unsafe {
            let ptr = self.data.as_ptr();
            for i in 0..len - 1 {
                let first = *ptr.add(i);
                let second = *ptr.add(i + 1);
                total = total.wrapping_add(first).wrapping_add(second);
            }
        }
        total
    }
}

fn spawn_task(holder: Arc<Mutex<DataHolder>>) {
    let handle = thread::spawn(move || {
        let guard = holder.lock().unwrap();
        let sum = guard.compute();
        println!("Computed value: {}", sum);
    });
    handle.join().unwrap();
}

fn main() {
    let holder = Arc::new(Mutex::new(DataHolder { data: vec![1, 2, 3, 4, 5] }));
    spawn_task(holder.clone());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    // This test oracle verifies the correct behavior.
    // It instantiates the data structure with known input and asserts the computed sum.
    // For the vulnerable version, the out-of-bound read may lead to a panic or an incorrect value,
    // causing the test to fail. The fixed version produces the expected result.
    #[test]
    fn test_computation() {
        let holder = DataHolder { data: vec![1, 2, 3, 4, 5] };
        
        // Catch potential panics (which may happen with the vulnerable version)
        let result = panic::catch_unwind(|| holder.compute());
        assert!(result.is_ok(), "The computation panicked unexpectedly");
        let value = result.unwrap();
        // For input [1,2,3,4,5], the safe computation sums adjacent pairs:
        // (1+2) + (2+3) + (3+4) + (4+5) = 3 + 5 + 7 + 9 = 24.
        assert_eq!(value, 24, "The computed sum is incorrect");
    }
}
