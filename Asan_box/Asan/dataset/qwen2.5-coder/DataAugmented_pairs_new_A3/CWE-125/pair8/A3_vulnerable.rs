struct Buffer {
    data: Vec<u32>,
}

impl Buffer {
    fn new(size: usize) -> Self {
        Buffer {
            data: vec![0; size],
        }
    }

    fn unsafe_read(&self, idx: usize) -> u32 {
        unsafe { *self.data.get_unchecked(idx) }
    }

    pub fn read_value_public(&self, index: usize) -> Option<u32> {
        Some(self.unsafe_read(index))
    }
}