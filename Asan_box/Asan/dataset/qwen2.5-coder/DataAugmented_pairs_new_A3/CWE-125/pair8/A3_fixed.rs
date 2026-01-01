struct Buffer {
    data: Vec<u32>,
}

impl Buffer {
    fn new(size: usize) -> Self {
        Buffer {
            data: vec![0; size],
        }
    }

    fn safe_read(&self, idx: usize) -> Option<u32> {
        if idx < self.data.len() {
            unsafe { Some(*self.data.get_unchecked(idx)) }
        } else {
            None
        }
    }

    pub fn read_value_public(&self, index: usize) -> Option<u32> {
        self.safe_read(index)
    }
}