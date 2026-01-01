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