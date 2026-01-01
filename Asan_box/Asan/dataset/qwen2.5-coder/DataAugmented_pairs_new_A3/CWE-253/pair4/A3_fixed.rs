struct Handler {
    buffer: [u8; 64],
}

impl Handler {
    fn new() -> Self {
        Handler { buffer: [0; 64] }
    }

    unsafe fn operation(&mut self, input: &[u8]) -> Result<(), String> {
        if input.len() > self.buffer.len() {
            return Err("Input exceeds buffer capacity".to_string());
        }
        self.buffer.copy_from_slice(input);
        Ok(())
    }

    pub fn process(&mut self, input: &[u8]) -> Result<(), String> {
        unsafe { self.operation(input)?; }
        Ok(())
    }
}