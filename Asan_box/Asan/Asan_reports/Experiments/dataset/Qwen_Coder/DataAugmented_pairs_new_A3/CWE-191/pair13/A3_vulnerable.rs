pub struct Counter {
    value: u32,
}

impl Counter {
    pub unsafe fn reduce(&mut self, amount: u32) {
        self.value = self.value - amount;
    }
}

pub fn execute(amount: u32) -> u32 {
    let mut counter = Counter { value: 10 };
    unsafe {
        counter.reduce(amount);
    }
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
