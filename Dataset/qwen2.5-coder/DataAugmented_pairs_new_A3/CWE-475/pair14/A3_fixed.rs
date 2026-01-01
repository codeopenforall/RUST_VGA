struct MemoryBlock {
    buffer: Vec<i32>,
}

impl MemoryBlock {
    fn new() -> Self {
        MemoryBlock {
            buffer: vec![10, 20, 30, 40, 50],
        }
    }

    pub fn safe_entry(&self, offset: usize, count: usize) -> Result<i32, &'static str> {
        self.validated_sum(offset, count)
    }

    fn validated_sum(&self, offset: usize, count: usize) -> Result<i32, &'static str> {
        if offset.checked_add(count).filter(|&sum| sum <= self.buffer.len()).is_none() {
            return Err("Invalid offset or count");
        }
        let slice = &self.buffer[offset..offset + count];
        Ok(slice.iter().sum())
    }
}