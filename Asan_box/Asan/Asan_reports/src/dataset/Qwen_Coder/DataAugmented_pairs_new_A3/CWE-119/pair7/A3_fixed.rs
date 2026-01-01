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

//////////////////// Oracle Test ////////////////////
// This oracle function is intended to serve as a single reproducible test.
// It verifies that processing an input that exceeds the internal buffer's boundary
// results in an error. In the vulnerable version, the operation would erroneously succeed
// (and corrupt internal memory), causing the test to fail.
#[cfg(test)]
mod tests {
    use super::Data;

    #[test]
    fn test_oracle() {
        let mut obj = Data::new();
        let input = vec![1u8; 100]; // Oversized input
        let result = obj.operate(&input);
        // The expected correct behavior is for the operation to return an error.
        assert!(result.is_err(), "Operation must reject oversized input");
    }
}
