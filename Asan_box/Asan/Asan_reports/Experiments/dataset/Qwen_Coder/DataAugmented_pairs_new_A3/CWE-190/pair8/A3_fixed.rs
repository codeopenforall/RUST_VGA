struct Processor {
    factor: u32,
}

impl Processor {
    fn calculate(&self, input: u32) -> Result<u32, &'static str> {
        let prod: u64 = (input as u64).wrapping_mul(self.factor as u64);
        if prod > u32::MAX as u64 {
            Err("Overflow detected")
        } else {
            Ok(prod as u32)
        }
    }

    fn check(&self, input: u32) -> Result<u32, &'static str> {
        self.calculate(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overflow_detection() {
        // Using the same triggering input for overflow.
        let processor = Processor { factor: 70000 };
        let input = 70000;
        let result = processor.check(input);
        // In the safe implementation the multiplication must trigger an overflow error.
        assert!(result.is_err(), "Expected an overflow error, but got {:?}", result);
    }
}
