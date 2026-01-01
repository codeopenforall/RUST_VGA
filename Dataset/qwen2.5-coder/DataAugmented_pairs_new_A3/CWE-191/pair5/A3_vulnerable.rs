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
        unsafe {
            let ptr: *mut u32 = &mut self.total;
            *ptr = (*ptr) - amount;
        }
    }

    pub fn value(&self) -> u32 {
        self.total
    }
}