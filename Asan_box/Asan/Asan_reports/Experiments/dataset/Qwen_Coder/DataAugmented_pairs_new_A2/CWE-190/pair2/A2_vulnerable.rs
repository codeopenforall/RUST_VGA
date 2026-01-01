#![feature(core_intrinsics)]
use std::intrinsics::unchecked_mul;
use std::sync::{Arc, Mutex};
use std::thread;

struct Processor;

impl Processor {
    fn calculate_product(nums: &[u64]) -> Result<u64, &'static str> {
        let mut product: u64 = 1;
        let shared = Arc::new(Mutex::new(product));
        let mut handles = vec![];
        for &num in nums {
            let shared_clone = Arc::clone(&shared);
            let handle = thread::spawn(move || {
                let mut val = shared_clone.lock().unwrap();
                unsafe {
                    *val = unchecked_mul(*val, num);
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        let final_product = *shared.lock().unwrap();
        if final_product == 0 {
            Err("Overflow detected")
        } else {
            Ok(final_product)
        }
    }
}

fn main() {
    let values = [9223372036854775808_u64, 4_u64];
    match Processor::calculate_product(&values) {
        Ok(result) => {
            println!("Computed product: {}", result);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Processor;

    #[test]
    fn test_oracle() {
        // The input that in its correct handling should detect an overflow.
        let values = [9223372036854775808_u64, 4_u64];
        let result = Processor::calculate_product(&values);
        // The expected behavior is to detect an overflow and return an error.
        assert!(result.is_err(), "Expected an overflow error, but got a valid result.");
    }
}
