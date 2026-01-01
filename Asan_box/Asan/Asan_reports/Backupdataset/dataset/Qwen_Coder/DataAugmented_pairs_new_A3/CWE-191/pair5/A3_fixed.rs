struct Accumulator {
    total: u32,
}

impl Accumulator {
    pub fn new(initial_value: u32) -> Self {
        Accumulator {
            total: initial_value,
        }
    }

    pub fn update(&mut self, amount: u32) {
        self.total = self.total.checked_sub(amount).unwrap_or(0);
    }

    pub fn value(&self) -> u32 {
        self.total
    }
}