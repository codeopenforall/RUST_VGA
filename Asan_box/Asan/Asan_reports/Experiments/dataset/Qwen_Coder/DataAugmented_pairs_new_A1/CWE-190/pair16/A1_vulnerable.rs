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
    use std::process::Command;
    use std::env;

    #[test]
    fn test_oracle() {
        // This test oracle expects the binary to exit with code 0 if the computation is correct.
        // In the vulnerable version the overflow causes an incorrect result and a non-zero exit code.
        // In the fixed version the correct product is computed and the process exits with 0.
        let current_exe = env::args().next().expect("Failed to get current executable");
        let output = Command::new(current_exe)
            .output()
            .expect("Failed to execute process");

        assert_eq!(
            output.status.code(),
            Some(0),
            "Test oracle failed: Unexpected exit code"
        );
    }
}
