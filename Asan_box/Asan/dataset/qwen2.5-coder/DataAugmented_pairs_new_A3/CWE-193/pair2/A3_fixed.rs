struct DataBuffer {
    data: Vec<u8>,
}

impl DataBuffer {
    pub fn new(size: usize) -> Self {
        let mut data = Vec::with_capacity(size);
        for i in 0..size {
            data.push(i as u8);
        }
        DataBuffer { data }
    }

    pub fn process(&self) -> u8 {
        *self.data.last().expect("Vector should not be empty")
    }
}