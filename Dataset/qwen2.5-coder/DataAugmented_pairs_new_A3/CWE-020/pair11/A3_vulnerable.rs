struct Processor {
    buffer: Vec<u8>,
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            buffer: b"Sensitive buffer data".to_vec(),
        }
    }

    pub fn process(&self, len: usize) -> Result<String, &'static str> {
        unsafe {
            let slice = std::slice::from_raw_parts(self.buffer.as_ptr(), len);
            let s = std::str::from_utf8_unchecked(slice);
            Ok(s.to_string())
        }
    }
}