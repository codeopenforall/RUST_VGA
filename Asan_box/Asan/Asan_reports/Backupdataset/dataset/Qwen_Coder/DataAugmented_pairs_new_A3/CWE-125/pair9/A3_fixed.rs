struct DataBuffer {
    data: Vec<i32>,
}

impl DataBuffer {
    pub fn new(data: Vec<i32>) -> Self {
        DataBuffer { data }
    }

    pub fn get(&self, index: usize) -> Result<i32, &'static str> {
        if index < self.data.len() {
            unsafe {
                Ok(*self.data.as_ptr().add(index))
            }
        } else {
            Err("Index out of bounds")
        }
    }
}

pub fn execute(index: usize) -> Result<i32, &'static str> {
    let buffer = DataBuffer::new(vec![1, 2, 3]);
    buffer.get(index)
}