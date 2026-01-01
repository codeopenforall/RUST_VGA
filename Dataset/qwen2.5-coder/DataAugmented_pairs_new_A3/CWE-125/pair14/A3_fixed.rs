struct Buffer {
    buffer: Vec<u8>,
}

impl Buffer {
    pub fn new(data: Vec<u8>) -> Self {
        Buffer { buffer: data }
    }

    pub fn get(&self, idx: usize) -> Option<u8> {
        if idx < self.buffer.len() {
            Some(self.buffer[idx])
        } else {
            None
        }
    }
}

fn run_op() -> Option<u8> {
    let buffer = Buffer::new(vec![1, 2, 3]);
    buffer.get(3)
}