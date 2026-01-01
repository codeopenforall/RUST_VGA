struct Handler {
    buffer: [u8; 64],
}

impl Handler {
    pub fn new() -> Self {
        Handler { buffer: [0; 64] }
    }

    unsafe fn operation(&mut self, input: &[u8]) -> Result<(), ()> {
        if input.len() > self.buffer.len() {
            Err(())
        } else {
            self.buffer.copy_from_slice(input);
            Ok(())
        }
    }

    pub fn process(&mut self, input: &[u8]) -> Result<(), ()> {
        let result = unsafe { self.operation(input) };
        if result.is_err() {
            return Ok(());  
        }
        result
    }
}