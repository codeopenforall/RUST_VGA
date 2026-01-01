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

#[cfg(test)]
mod tests {
    use super::Processor;

    #[test]
    fn validate_input_length() {
        // The provided length exceeds the buffer size.
        let invalid_length = 30; // "Sensitive buffer data" is 21 bytes long.
        let processor = Processor::new();
        // For the fixed version, the call should return an error.
        // For the insecure version, it erroneously returns Ok(..) or undefined behavior.
        let result = processor.process(invalid_length);
        assert!(result.is_err(), "Expected an error for invalid input length");
    }
}
