pub struct Processor;

impl Processor {
    pub fn process(&self, input: &str) -> Result<String, String> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 2 {
            return Err("Invalid input format".to_owned());
        }

        let expected_len: usize = match parts[0].parse() {
            Ok(len) => len,
            Err(_) => return Err("Invalid length format".to_owned()),
        };

        let data = parts[1];
        if expected_len > data.len() {
            return Err("Length exceeds payload size".to_owned());
        }

        let subslice = &data.as_bytes()[..expected_len];
        let result = std::str::from_utf8(subslice).map_err(|_| "Payload is not valid UTF-8")?;
        Ok(result.to_owned())
    }
}

////////////////////////////////////////////////////////////
// Test Oracle Function for Vulnerability Validation
////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::Processor;

    // The test passes for the corrected version because it will return an error when:
    //  - The specified length exceeds the actual payload length.
    // For the vulnerable version, the unsafe block might produce unpredictable results,
    // causing the test to fail consistently.
    #[test]
    fn test_input_length_validation() {
        let processor = Processor;
        // Input where the expected length (10) is greater than the payload length ("hello" is 5).
        let input = "10 hello";
        let result = processor.process(input);
        // For the fixed version, we expect an explicit error message.
        assert!(result.is_err(), "Expected an error due to length exceeding payload size");
        assert_eq!(result.unwrap_err(), "Length exceeds payload size");
    }
}
////////////////////////////////////////////////////////////
// End of Test Oracle Function
////////////////////////////////////////////////////////////
