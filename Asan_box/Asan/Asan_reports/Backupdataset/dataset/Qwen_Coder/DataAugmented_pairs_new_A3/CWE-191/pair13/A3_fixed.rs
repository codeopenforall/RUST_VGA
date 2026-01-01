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