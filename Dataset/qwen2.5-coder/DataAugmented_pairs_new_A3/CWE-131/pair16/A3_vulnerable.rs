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