struct Processor {
    buffer: Vec<u8>,
}

impl Processor {
    fn new() -> Self {
        Processor {
            buffer: b"Sensitive buffer data".to_vec(),
        }
    }

    pub fn process(&self, len: usize) -> Result<String, &'static str> {
        if len > self.buffer.len() {
            return Err("Invalid length: exceeds buffer size");
        }
        let s = std::str::from_utf8(&self.buffer[..len]).map_err(|_| "UTF8 conversion error")?;
        Ok(s.to_string())
    }
}