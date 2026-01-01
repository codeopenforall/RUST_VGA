pub struct Data {
    buffer: Vec<u8>,
}

impl Data {
    pub fn new(input: &str) -> Self {
        let buf_size = std::mem::size_of_val(&input); // Vulnerable line
        let mut buffer = vec![0; buf_size];
        buffer[..input.len()].copy_from_slice(input.as_bytes());
        Data { buffer }
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.buffer).unwrap()
    }
}

///////////////////////////////////////////////////////////////
// Test Oracle Function
///////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::Data;

    #[test]
    fn test_data_copy() {
        let input = "This is a somewhat long input string causing miscalculation.";
        let data = Data::new(input);
        // The vulnerable version will only copy a truncated portion (e.g., 16 bytes)
        // whereas the fixed version should copy the entire string. 
        // If the output is not equal to the input, then the test fails.
        assert_eq!(data.as_str(), input, "Buffer content does not match input string");
    }
}
