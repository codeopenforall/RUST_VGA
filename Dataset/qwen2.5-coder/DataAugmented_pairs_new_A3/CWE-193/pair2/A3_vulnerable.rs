struct DataBuffer {
    data: Vec<u8>,
}

impl DataBuffer {
    pub fn new(size: usize) -> Self {
        let mut data = Vec::with_capacity(size);
        unsafe { data.set_len(size); }
        for i in 0..size {
            data[i] = i as u8;
        }
        DataBuffer { data }
    }

    pub fn process(&self) -> u8 {
        unsafe {
            *self.data.get_unchecked(self.data.len())
        }
    }
}