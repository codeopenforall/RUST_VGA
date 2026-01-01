struct Processor {
    buffer: Vec<u8>,
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            buffer: vec![0; 10], // Example buffer of size 10
        }
    }

    pub fn process(&self, len_str: &str) -> String {
        let len: usize = len_str.parse().unwrap_or(0);

        // UNSAFE USAGE: Intentionally using pointer arithmetic and unchecked UTF-8 conversion.
        // Vulnerability: No validation is done to ensure len is within the bounds of the buffer.
        let ptr = self.buffer.as_ptr();
        let slice = unsafe { std::slice::from_raw_parts(ptr, len) };

        // Simulate processing
        String::from_utf8(slice.to_vec()).unwrap_or_else(|_| "Error converting to UTF-8".to_string())
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
