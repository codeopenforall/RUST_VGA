pub struct Calculation {
    base: i32,
    current: i32,
    percent: i32,
}

impl Calculation {
    pub fn new(base: i32, current: i32) -> Self {
        Calculation { base, current, percent: 0 }
    }

    pub unsafe fn calculate(&mut self) {
        let percent_ptr: *mut i32 = &mut self.percent;
        *percent_ptr = self.current - self.base * 100 / self.base;
    }
}

#[cfg(test)]
mod tests {
    use super::Calculation;

    fn execute_calculation(calculator: &mut Calculation) {
        unsafe {
            calculator.calculate();
        }
    }

    #[test]
    fn percentage_test() {
        // Prepare the calculation with base 200 and current 250.
        // The expected correct percentage increase is 25.
        let mut calc = Calculation::new(200, 250);
        execute_calculation(&mut calc);
        assert_eq!(calc.percent, 25, "Expected 25% increase, but got a different value.");
    }
}
