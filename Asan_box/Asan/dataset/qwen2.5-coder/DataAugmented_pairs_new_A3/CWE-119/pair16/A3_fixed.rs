struct Buffer {
    data: Vec<u32>,
}

impl Buffer {
    pub fn new(size: usize) -> Self {
        Buffer {
            data: vec![0; size],
        }
    }

    pub fn update(&mut self, idx: usize, val: u32) -> Result<(), &'static str> {
        if idx >= self.data.len() {
            return Err("index out of bounds");
        }
        self.data[idx] = val;
        Ok(())
    }
}

fn run() -> Result<(), &'static str> {
    let mut buf = Buffer::new(10);
    buf.update(10, 100)?;
    Ok(())
}