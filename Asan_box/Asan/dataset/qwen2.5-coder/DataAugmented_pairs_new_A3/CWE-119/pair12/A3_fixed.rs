pub struct MemoryBlock {
    data: [u8; 10],
    flag: u8,
}

impl MemoryBlock {
    pub fn update(&mut self, index: usize, value: u8) {
        if index < self.data.len() {
            unsafe {
                let ptr = self.data.as_mut_ptr();
                *ptr.add(index) = value;
            }
        } else {
            // Do nothing if the index is out of bounds
        }
    }
}