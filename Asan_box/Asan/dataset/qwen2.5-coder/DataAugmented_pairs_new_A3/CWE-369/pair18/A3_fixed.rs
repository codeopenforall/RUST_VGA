struct Engine {
    value: i32,
}

impl Engine {
    pub fn new(value: i32) -> Self {
        Engine { value }
    }

    pub fn compute(&self, divisor: i32) -> i32 {
        if divisor == 0 {
            self.value
        } else {
            self.value / divisor
        }
    }
}