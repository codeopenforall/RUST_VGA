use std::ops::Mul;

#[derive(Debug)]
pub struct VulnerableMultiplier;

impl Multiply for VulnerableMultiplier {
    fn multiply(&self, a: u32, b: u32) -> Result<u32, &'static str> {
        unsafe {
            let result = (a as u64 * b as u64) as u32;
            Ok(result)
        }
    }
}

pub trait Multiply {
    fn multiply(&self, a: u32, b: u32) -> Result<u32, &'static str>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_overflow() {
        let multiplier = VulnerableMultiplier;
        let a = std::u32::MAX;
        let b = 2;
        let result = multiplier.multiply(a, b);
        assert!(result.is_err(), "Overflow was not detected: {:?}", result);
    }
}

/*
   Test Oracle Function:
   This function drives the multiplication on potentially overflowing inputs.
   It asserts that an error is returned when the multiplication would overflow.
   For the vulnerable implementation, the assertion will fail because it returns a wrapped value.
   For the corrected version, the assertion passes.
*/
#[allow(dead_code)]
pub fn test_oracle<M: Multiply>(processor: &M) {
    let a = std::u32::MAX;
    let b = 2;
    let result = processor.multiply(a, b);
    assert!(
        result.is_err(),
        "Test failed: Overflow not detected, result was: {:?}",
        result
    );
}
