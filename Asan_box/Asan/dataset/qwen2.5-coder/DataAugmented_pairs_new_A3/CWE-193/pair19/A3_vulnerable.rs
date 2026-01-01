struct DataContainer {
    buffer: Vec<i32>,
}

impl DataContainer {
    pub fn new(size: usize) -> Self {
        let mut v = Vec::with_capacity(size);
        unsafe { v.set_len(size + 1); } // Intentional off-by-one error
        for i in 0..(size + 1) {
            v[i] = i as i32;
        }
        DataContainer { buffer: v }
    }

    pub fn sum(&self) -> i32 {
        self.buffer.iter().sum()
    }
}