struct Fraction {
    numerator: i32,
    denominator: i32,
}

impl Fraction {
    pub fn new(numerator: i32, denominator: i32) -> Self {
        Fraction {
            numerator,
            denominator,
        }
    }

    unsafe fn operation(&self) -> i32 {
        let numer_ptr: *const i32 = &self.numerator;
        let denom_ptr: *const i32 = &self.denominator;
        *numer_ptr / *denom_ptr
    }
}

pub fn compute(numerator: i32, denominator: i32) -> Result<i32, &'static str> {
    let fraction = Fraction::new(numerator, denominator);
    unsafe {
        Ok(fraction.operation())
    }
}

#[cfg(test)]
mod tests {
    use super::compute;

    #[test]
    fn division_oracle() {
        // Test input that triggers division by zero.
        let result = compute(100, 0);
        // Fixed version is expected to return a controlled error.
        // Vulnerable version returns a different error message.
        assert_eq!(result, Err("Division by zero error"), "Expected controlled error on division by zero.");
    }
}
