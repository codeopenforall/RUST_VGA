pub struct Data {
    pub counter: u32,
}

impl Data {
    pub fn new(initial_value: u32) -> Self {
        Data {
            counter: initial_value,
        }
    }

    pub fn subtract_val(&mut self, val: i32) {
        // Intentionally vulnerable to CWE-191: Integer Underflow
        self.counter = self.counter.wrapping_sub(val as u32);
    }
}