pub struct Calculator;

impl Calculator {
    pub fn compute(&self, n: u32) -> Result<u32, &'static str> {
        n.checked_mul(100000).ok_or("overflow")
    }
}

#[cfg(test)]
mod tests {
    use super::Calculator;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_overflow() {
        let calc = Calculator;
        let input = 50000u32;
        let shared_calc = Arc::new(calc);
        let calc_clone = Arc::clone(&shared_calc);

        let handler = thread::spawn(move || {
            calc_clone.compute(input)
        });

        let result = handler.join().unwrap();
        // In the fixed implementation, the multiplication will detect overflow and return an error.
        // In the vulnerable implementation, an incorrect wrapped value is returned.
        assert!(result.is_err(), "Expected an overflow error, but got {:?}", result);
    }
}
