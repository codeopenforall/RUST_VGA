struct Buffer {
    data: Vec<i32>,
}

impl Buffer {
    pub fn new(data: Vec<i32>) -> Self {
        Buffer { data }
    }

    pub fn fetch(&self, idx: usize) -> i32 {
        unsafe {
            *self.data.get_unchecked(idx - 1)
        }
    }
}