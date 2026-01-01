struct Data {
    counter: u32,
}

impl Data {
    pub fn new(initial_value: u32) -> Self {
        Data {
            counter: initial_value,
        }
    }

    pub fn subtract_val(&mut self, val: i32) {
        if val < 0 {
            return;
        }
        self.counter = self.counter.wrapping_sub(val as u32);
    }
}