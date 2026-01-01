pub struct Manager {
    len: usize,
}

impl Manager {
    pub fn new(len: usize) -> Self {
        Manager { len }
    }

    pub fn process(&mut self, idx: usize) -> Result<(), &'static str> {
        if idx >= self.len {
            return Err("Index out-of-bounds");
        }
        // Simulate some processing
        Ok(())
    }
}