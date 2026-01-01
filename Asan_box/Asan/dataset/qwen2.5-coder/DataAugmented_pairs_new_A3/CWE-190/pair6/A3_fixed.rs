struct Calculator {
    factor: i32,
}

impl Calculator {
    fn new() -> Self {
        Calculator { factor: 2 }
    }

    pub fn compute(&self, base: i32, input: u32) -> i32 {
        let safe_input = i32::try_from(input).unwrap_or(i32::MAX);
        let addition = safe_input.saturating_mul(self.factor);
        base.saturating_add(addition)
    }
}