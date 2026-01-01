pub struct Data {
    buffer: Vec<u8>,
}

impl Data {
    pub fn new(input: &str) -> Self {
        let buf_size = input.len();
        let mut buffer = vec![0; buf_size];
        buffer.copy_from_slice(input.as_bytes());
        Data { buffer }
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.buffer).unwrap()
    }
}