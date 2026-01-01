struct Container {
    buffer: Vec<i32>,
}

impl Container {
    pub fn compute(&self) -> i32 {
        unsafe {
            *self.buffer.get_unchecked(self.buffer.len())
        }
    }
}