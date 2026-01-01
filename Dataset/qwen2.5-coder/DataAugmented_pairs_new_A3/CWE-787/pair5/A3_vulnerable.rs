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
        unsafe {
            let ptr = self.data.as_mut_ptr();
            *ptr.add(index) = value;
            self.data.set_len(self.data.len() + 1);
        }
        self.data.iter().sum()
    }
}