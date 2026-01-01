pub struct Calculator {
    count: u32,
}

impl Calculator {
    pub fn new(initial_count: u32) -> Self {
        Calculator {
            count: initial_count,
        }
    }

    pub fn decrement(&mut self, val: u32) {
        unsafe {
            let ptr = &mut self.count as *mut u32;
            *ptr = *ptr - val;
        }
    }

    pub fn get_count(&self) -> u32 {
        self.count
    }
}

pub fn execute_calculation() -> u32 {
    let mut calc = Calculator::new(0);
    calc.decrement(1);
    calc.get_count()
}

#[cfg(test)]
mod tests {
    // Depending on which implementation is being tested (vulnerable or fixed),
    // this test should pass only when the underlying subtraction safely avoids underflow.
    // For the vulnerable implementation, subtracting 1 from 0 will yield u32::MAX,
    // causing the test to fail.
    use super::execute_calculation;

    #[test]
    fn test_prevent_underflow() {
        let result = execute_calculation();
        // In the corrected implementation, the expected final value is 0.
        // In the vulnerable version, the result would be 4294967295.
        assert_eq!(result, 0, "Underflow detected: result is {}", result);
    }
}
