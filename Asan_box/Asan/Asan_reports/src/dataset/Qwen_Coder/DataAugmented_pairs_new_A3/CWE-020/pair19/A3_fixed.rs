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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extraction_validation() {
        // Use an input string known to be 13 bytes long.
        let content = "Hello, World!";
        let proc_inst = Processor::new(content);

        // The chosen indices (offset=7, length=10) exceed the string bounds.
        // The test expects an error.
        let result = proc_inst.process(7, 10);
        assert!(result.is_err(), "Expected error due to out-of-bound indices");
    }
}
