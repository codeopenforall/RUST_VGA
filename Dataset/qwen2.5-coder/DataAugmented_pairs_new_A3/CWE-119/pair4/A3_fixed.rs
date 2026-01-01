struct ManagedBuffer {
    data: Vec<u8>,
}

impl ManagedBuffer {
    pub fn new(size: usize) -> Self {
        ManagedBuffer {
            data: vec![0; size],
        }
    }

    pub fn update(&mut self, src: &[u8], start: usize, len: usize) -> Result<(), &'static str> {
        if start.checked_add(len).filter(|&sum| sum <= self.data.len()).is_none() {
            return Err("Destination buffer index out of bounds");
        }
        if len > src.len() {
            return Err("Source slice does not have enough elements");
        }
        self.data[start..start + len].copy_from_slice(&src[..len]);
        Ok(())
    }

    pub fn get(&self) -> &[u8] {
        &self.data
    }
}