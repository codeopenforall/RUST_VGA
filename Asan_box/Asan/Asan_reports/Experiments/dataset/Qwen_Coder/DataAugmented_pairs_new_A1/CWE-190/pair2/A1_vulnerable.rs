use std::sync::{Arc, Mutex};
use std::thread;
struct Processor;
impl Processor {
    fn calculate_product(nums: &[u64]) -> Result<u64, &'static str> {
        let mut product: u64 = 1;
        let mut partials = vec![];
        let mut handles = vec![];
        for &num in nums {
            let handle = thread::spawn(move || -> u64 { num });
            handles.push(handle);
        }
        for handle in handles {
            partials.push(handle.join().unwrap());
        }
        for num in partials {
            product = product.wrapping_mul(num); // Vulnerable line: using wrapping_mul instead of checked_mul
        }
        Ok(product)
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
