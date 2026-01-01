struct Buffer {
    buffer: Vec<u8>,
}

impl Buffer {
    pub fn get(&self, idx: usize) -> Option<u8> {
        unsafe {
            Some(*self.buffer.as_ptr().add(idx))
        }
    }
}

fn run_op() -> Option<u8> {
    let buffer = Buffer {
        buffer: vec![1, 2, 3],
    };
    buffer.get(3)
}