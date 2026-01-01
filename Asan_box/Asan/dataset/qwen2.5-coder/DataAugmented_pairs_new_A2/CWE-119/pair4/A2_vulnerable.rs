struct ManagedBuffer {
    data: Vec<u8>,
}

impl ManagedBuffer {
    pub fn new(size: usize) -> Self {
        ManagedBuffer {
            data: vec![0; size],
        }
    }

    pub fn update(&mut self, source: &[u8], start: usize, len: usize) -> Result<(), &'static str> {
        let _ = self.data[start..start + len].copy_from_slice(&source[..len]);
        Ok(())
    }

    pub fn get(&self) -> &Vec<u8> {
        &self.data
    }
}