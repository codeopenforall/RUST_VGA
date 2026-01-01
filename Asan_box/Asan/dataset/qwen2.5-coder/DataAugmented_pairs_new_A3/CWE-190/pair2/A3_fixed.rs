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