struct BufferHandler {
    buffer: Vec<i32>,
}

impl BufferHandler {
    pub fn new() -> Self {
        BufferHandler { buffer: Vec::new() }
    }

    pub fn load(&mut self, data: &[i32]) {
        self.buffer.extend_from_slice(data);
    }

    pub fn calculate(&self) -> i32 {
        let len = self.buffer.len();
        let mut sum = 0;
        for i in 0..len {
            sum += self.buffer[i];
        }
        sum
    }
}