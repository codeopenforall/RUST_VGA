struct ManagedBuffer {
    data: Vec<u8>,
}

impl ManagedBuffer {
    pub fn new(size: usize) -> Self {
        ManagedBuffer {
            data: vec![0; size],
        }
    }

    pub fn update(&mut self, src: &[u8], start: usize, len: usize) -> Result<(), &'static str> {
        if start.checked_add(len).filter(|&sum| sum <= self.data.len()).is_none() {
            return Err("Destination buffer index out of bounds");
        }
        if len > src.len() {
            return Err("Source slice does not have enough elements");
        }
        self.data[start..start + len].copy_from_slice(&src[..len]);
        Ok(())
    }

    pub fn get(&self) -> &[u8] {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    // Import the common structures and implementations from the surrounding module.
    use super::*;

    // The test oracle calls the update method with parameters that attempt to write outside of the 
    // allocated buffer. In the corrected code, the update method must catch this and return an error.
    // For the vulnerable version, update would erroneously return Ok(()) and perform an unsafe write.
    #[test]
    fn test_oracle() {
        let mut buf = ManagedBuffer::new(10);
        let source = vec![1, 2, 3, 4, 5];
        // Parameters: starting at index 8, trying to copy 5 elements (8+5=13, which is out of bounds)
        let result = buf.update(&source, 8, 5);
        assert!(result.is_err(), "The update function should detect the out-of-bound operation");
    }
}
