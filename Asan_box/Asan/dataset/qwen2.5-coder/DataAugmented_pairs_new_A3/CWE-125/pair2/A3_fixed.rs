pub struct Buffer {
    data: Vec<i32>,
}

impl Buffer {
    pub fn new(data: Vec<i32>) -> Self {
        Buffer { data }
    }

    pub fn access(&self, idx: usize) -> Option<i32> {
        self.data.get(idx).cloned()
    }
}