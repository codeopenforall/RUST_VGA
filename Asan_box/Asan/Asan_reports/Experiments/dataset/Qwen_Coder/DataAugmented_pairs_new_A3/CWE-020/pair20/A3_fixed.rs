struct Processor {
    buffer: Vec<u8>,
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            buffer: vec![0; 10], // Example buffer size
        }
    }

    pub fn process(&self, len_str: &str) -> String {
        match len_str.parse::<usize>() {
            Ok(len) => {
                // FIX: Validate that the provided length does not exceed the buffer size.
                if len > self.buffer.len() {
                    return "Invalid input size".to_string();
                }
                // Safe conversion: Using pointer arithmetic only after ensuring bounds.
                let ptr = self.buffer.as_ptr();
                let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
                // Further processing can be done here with the slice
                format!("Processed {} bytes", slice.len())
            }
            Err(_) => "Invalid input format".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Processor;

    // This oracle test calls the process method with a length string ("15")
    // On the vulnerable version, this would lead to undefined behavior or incorrect results.
    // On the corrected version, it will return the error message "Invalid input size".
    #[test]
    fn test_input_validation() {
        let proc_inst = Processor::new();
        let result = proc_inst.process("15");
        // The test expects the properly validated response.
        assert_eq!(result, "Invalid input size", "The process function did not validate the input length properly");
    }
}
