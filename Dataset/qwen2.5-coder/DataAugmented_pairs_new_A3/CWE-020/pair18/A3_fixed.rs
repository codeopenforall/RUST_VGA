struct Holder {
    bytes: Vec<u8>,
}

impl Holder {
    pub fn new(bytes: &[u8]) -> Result<Self, &'static str> {
        Ok(Holder {
            bytes: bytes.to_vec(),
        })
    }

    pub fn process(&self, offset: usize, count: usize) -> String {
        let available = self.bytes.len().saturating_sub(offset);
        let valid_count = if count > available { available } else { count };
        let ptr = self.bytes.as_ptr().add(offset);
        let slice = unsafe { std::slice::from_raw_parts(ptr, valid_count) };
        unsafe { std::str::from_utf8_unchecked(slice) }.to_string()
    }
}