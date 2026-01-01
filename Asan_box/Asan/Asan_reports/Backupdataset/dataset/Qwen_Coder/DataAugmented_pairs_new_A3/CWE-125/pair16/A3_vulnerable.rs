struct DataStore {
    buffer: [u8; 5],
}

impl DataStore {
    pub fn new() -> Self {
        DataStore {
            buffer: [0, 1, 2, 3, 4],
        }
    }

    pub fn fetch(&self, position: usize) -> u8 {
        unsafe { *self.buffer.get_unchecked(position) }
    }
}