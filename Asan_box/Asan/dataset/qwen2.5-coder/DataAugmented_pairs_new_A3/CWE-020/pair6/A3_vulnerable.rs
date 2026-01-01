use std::str;

pub struct Processor;

impl Processor {
    pub fn process(&self, input: &str) -> Result<String, String> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 2 {
            return Err("Invalid input format".to_owned());
        }

        let expected_len: usize = parts[0].parse().map_err(|_| "Invalid length".to_owned())?;
        let data = parts[1];

        unsafe {
            let ptr = data.as_ptr();
            let unslice = std::slice::from_raw_parts(ptr, expected_len);
            let result = str::from_utf8_unchecked(unslice);
            Ok(result.to_owned())
        }
    }
}