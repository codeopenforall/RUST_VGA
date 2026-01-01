struct Buffer {
    data: Vec<i32>,
}

impl Buffer {
    pub fn new(data: Vec<i32>) -> Self {
        Buffer { data }
    }

    pub fn fetch(&self, idx: usize) -> i32 {
        if idx == 0 || idx > self.data.len() {
            panic!("Index out of bounds");
        }
        self.data[idx - 1]
    }
}