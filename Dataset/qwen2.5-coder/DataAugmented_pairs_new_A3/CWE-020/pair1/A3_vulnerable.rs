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