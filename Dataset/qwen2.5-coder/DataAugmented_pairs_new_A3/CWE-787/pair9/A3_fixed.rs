struct DataHolder {
    buffer: Vec<u32>,
}

impl DataHolder {
    pub fn new() -> Self {
        DataHolder {
            buffer: Vec::new(),
        }
    }

    pub fn process(&mut self, input: &[u32]) {
        if input.len() > self.buffer.capacity() {
            self.buffer = Vec::with_capacity(input.len());
        }
        self.buffer.extend_from_slice(input);
    }

    pub fn total(&self) -> u32 {
        self.buffer.iter().sum()
    }
}