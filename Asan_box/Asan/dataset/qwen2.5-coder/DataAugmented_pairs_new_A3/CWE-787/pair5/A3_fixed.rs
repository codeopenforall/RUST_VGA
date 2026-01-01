struct DataBuffer {
    data: Vec<i32>,
}

impl DataBuffer {
    pub fn new(size: usize) -> Self {
        DataBuffer {
            data: vec![0; size],
        }
    }

    pub fn process(&mut self, index: usize, value: i32) -> i32 {
        if index < self.data.len() {
            self.data[index] = value;
        }
        self.data.iter().sum()
    }
}