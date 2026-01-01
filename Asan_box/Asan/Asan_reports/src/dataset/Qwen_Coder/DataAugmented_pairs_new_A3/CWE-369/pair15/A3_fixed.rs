pub struct Engine {
    numerator: i32,
}

impl Engine {
    pub fn new(numerator: i32) -> Self {
        Engine { numerator }
    }

    pub fn exec(&self, divisor: i32) -> Result<i32, &'static str> {
        if divisor == 0 {
            return Err("division by zero");
        }
        Ok(self.numerator / divisor)
    }
}

#[cfg(test)]
mod tests {
    use super::Engine;
    use std::panic;

    #[test]
    fn division_by_zero_test() {
        // Create an instance with a non-zero numerator.
        let engine = Engine::new(50);
        // Execute the division with zero as divisor.
        // In the corrected implementation, exec should return Err.
        // In the insecure implementation, a division by zero panic occurs.
        let result = panic::catch_unwind(|| engine.exec(0));
        match result {
            // For the corrected code: no panic occurs, and exec returns an Err.
            Ok(res) => {
                assert!(res.is_err(), "Expected an error result for division by zero");
            },
            // For the vulnerable code: a panic is caught.
            Err(_) => {
                panic!("Unexpected panic occurred");
            }
        }
    }
}
