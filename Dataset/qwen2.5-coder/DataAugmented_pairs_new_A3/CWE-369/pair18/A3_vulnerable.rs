struct Engine {
    value: i32,
}

impl Engine {
    pub fn new(value: i32) -> Self {
        Engine { value }
    }

    pub fn compute(&self, divisor: i32) -> i32 {
        unsafe {
            let result = self.value / divisor;
            result
        }
    }
}