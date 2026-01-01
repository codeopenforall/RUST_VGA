struct DataStore {
    buffer: Vec<u8>,
}

impl DataStore {
    pub fn new() -> Self {
        DataStore {
            buffer: vec![0, 1, 2, 3, 4],
        }
    }

    pub fn fetch(&self, position: usize) -> u8 {
        self.buffer.get(position).copied().expect("Index out-of-bounds")
    }
}