struct Buffer {
    buffer: Vec<i32>,
}

impl Buffer {
    pub fn new(size: usize) -> Self {
        Buffer {
            buffer: vec![0; size],
        }
    }

    pub fn process(&mut self, value: i32) {
        unsafe {
            let ptr = self.buffer.as_mut_ptr();
            *ptr.add(0) = value; // Intentionally using index 0 for demonstration
        }
        assert!(self.buffer[0] < 1000, "Invariant violated: value must be less than 1000");
    }
}

pub fn process(value: i32) {
    let mut buffer = Buffer::new(1);
    buffer.process(value);
}