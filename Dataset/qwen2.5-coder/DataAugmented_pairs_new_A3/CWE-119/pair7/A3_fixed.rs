struct Data {
    buffer: Vec<u8>,
}

impl Data {
    pub fn new() -> Self {
        Data {
            buffer: vec![0u8; 50], // Example buffer size
        }
    }

    pub fn operate(&mut self, input: &[u8]) -> Result<(), &'static str> {
        if input.len() > self.buffer.len() {
            return Err("Input size exceeds buffer capacity");
        }
        // Simulate some operation
        self.buffer.copy_from_slice(input);
        Ok(())
    }

    pub fn check(&self) -> bool {
        // Placeholder for integrity check
        true
    }
}