use std::sync::{Arc, Mutex};
use std::thread;

pub struct Processor;

impl Processor {
    pub fn calculate_product(values: &[u64]) -> Result<u64, &'static str> {
        let mut product = 1_u64;
        let mut partials = vec![];

        for &num in values {
            let handle = thread::spawn(move || num);
            partials.push(handle.join().unwrap());
        }

        for num in partials {
            product = product.checked_mul(num).ok_or("overflow detected")?;
        }

        Ok(product)
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
