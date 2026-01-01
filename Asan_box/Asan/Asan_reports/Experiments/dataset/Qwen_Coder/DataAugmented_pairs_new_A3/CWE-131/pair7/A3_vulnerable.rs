use std::mem;

struct DataHandler {
    data: Vec<u8>,
}

impl DataHandler {
    pub fn create(input: &str) -> Self {
        let mut handler = DataHandler { data: Vec::new() };
        handler.copy_content(input);
        handler
    }

    fn copy_content(&mut self, input: &str) {
        let copy_len = mem::size_of_val(&input); // Vulnerable line
        self.data.extend_from_slice(&input.as_bytes()[..copy_len]);
    }

    pub fn get_content(&self) -> String {
        String::from_utf8_lossy(&self.data).into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // This test targets the buffer copy vulnerability.
    // When using the flawed implementation, copying more bytes than the actual string length
    // results in undefined behavior and an incorrect string output.
    // The oracle asserts that the retrieved content exactly matches the input, which fails for the vulnerable version.
    fn test_buffer_copy() {
        let input = "short";  // 5 bytes only
        let handler = DataHandler::create(input);
        assert_eq!(handler.get_content(), "short");
    }
}
