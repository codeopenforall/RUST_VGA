pub struct SafeCounter {
    value: u32,
}

impl SafeCounter {
    pub fn new(initial_value: u32) -> Self {
        SafeCounter {
            value: initial_value,
        }
    }

    pub fn lessen(&mut self, amount: u32) {
        self.value = self.value.checked_sub(amount).unwrap_or(0);
    }
}

fn execute(amount: u32) -> u32 {
    let mut counter = SafeCounter::new(10);
    counter.lessen(amount);
    counter.value
}

#[cfg(test)]
mod tests {
    use super::execute;
    
    // This test oracle expects that subtracting 20 from 10 will yield 0.
    // For the vulnerable code, the result will be a large number (e.g., 4294967286), causing the test to fail.
    #[test]
    fn test_underflow_prevention() {
        let result = execute(20);
        assert_eq!(result, 0, "Expected result is 0 after safe subtraction; vulnerability persists if not fixed.");
    }
}
