use std::fmt;

struct Processor;

impl Processor {
    pub fn process_input(&self, input: &[u8]) -> Result<String, &'static str> {
        if input.len() < 4 {
            return Err("Input too short for message");
        }

        let claimed = u32::from_le_bytes([input[0], input[1], input[2], input[3]]) as usize;
        let data_ptr = unsafe { input.as_ptr().add(4) };
        let message_slice = unsafe { std::slice::from_raw_parts(data_ptr, claimed) };
        let result_str = unsafe { std::str::from_utf8_unchecked(message_slice) };
        Ok(result_str.to_string())
    }
}

fn main() {}

//////////////// Test Oracle //////////////////
// The following test function is designed to enforce the contract that malformed inputs must be rejected.
// It should be run separately from the main application code.
// 
// The test supplies an input where the header claims a message length (100) that exceeds the actual data length.
// - For the secure version, process_input is expected to return an Err("Input too short for message").
// - For the vulnerable version, the function may panic or return an incorrect result.
//
// Note: This test is intended to be included in a separate test module.
#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn validate_input_length() {
        // Construct a malformed input:
        // Header claims 100 bytes, but only 2 message bytes are provided.
        let malformed_input = vec![100, 0, 0, 0, b'O', b'K'];
        let proc = Processor;
        
        // Use catch_unwind to capture potential panics in the vulnerable version.
        let result = panic::catch_unwind(|| proc.process_input(&malformed_input));

        // In the fixed version, the function should not panic and must return an error.
        assert!(result.is_ok(), "Function panicked unexpectedly.");
        let output = result.unwrap();
        assert!(output.is_err(), "Malformed input was not detected as invalid.");
        // Optionally, check for the specific error message.
        assert_eq!(output.err().unwrap(), "Input too short for message");
    }
}
