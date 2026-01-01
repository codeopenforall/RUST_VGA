pub struct Handler;

impl Handler {
    pub fn process(input: &str) -> Vec<u8> {
        // Intentionally using size_of_val which ignores the actual length of the string
        let size = std::mem::size_of_val(&input);
        input[..size].as_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::Handler;
    
    #[test]
    fn test_buffer_copy() {
        // The input string is deliberately chosen to exceed the size of a fat pointer.
        let input = "This is a test string that exceeds the typical pointer size";
        let result = Handler::process(input);
        // Expect the full string bytes to be copied.
        assert_eq!(result, input.as_bytes(), "Buffer copy does not match input");
    }
}
