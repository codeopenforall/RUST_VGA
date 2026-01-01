struct BufferProcessor {
    buffer: [u8; 10],
}

impl BufferProcessor {
    pub fn new() -> Self {
        BufferProcessor {
            buffer: [0; 10],
        }
    }

    pub fn update(&mut self, input: &[u8]) -> Result<(), &'static str> {
        if input.len() > self.buffer.len() {
            return Err("Input exceeds buffer capacity");
        }
        self.buffer.copy_from_slice(input);
        Ok(())
    }

    pub fn read(&self) -> &[u8] {
        &self.buffer
    }
}