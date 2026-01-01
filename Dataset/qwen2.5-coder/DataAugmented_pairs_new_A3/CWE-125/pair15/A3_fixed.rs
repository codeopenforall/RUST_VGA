struct Container {
    buffer: Vec<i32>,
}

impl Container {
    pub fn compute(&self) -> i32 {
        self.buffer.last().copied().expect("Buffer should not be empty")
    }
}