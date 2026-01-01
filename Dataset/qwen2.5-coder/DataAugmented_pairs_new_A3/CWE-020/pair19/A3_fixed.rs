struct Processor {
    content: Vec<u8>,
}

impl Processor {
    fn new(content: &str) -> Self {
        Processor {
            content: content.as_bytes().to_vec(),
        }
    }

    fn extract_section(&self, offset: usize, length: usize) -> Result<String, &'static str> {
        if offset.checked_add(length).filter(|&end| end <= self.content.len()).is_none() {
            return Err("Invalid bounds");
        }
        let slice = &self.content[offset..offset + length];
        Ok(slice.to_string())
    }

    fn process(&self, offset: usize, length: usize) -> Result<String, &'static str> {
        self.extract_section(offset, length)
    }
}