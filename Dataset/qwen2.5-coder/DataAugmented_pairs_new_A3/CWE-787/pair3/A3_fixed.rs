pub struct Buffer {
    data: Vec<i32>,
}

impl Buffer {
    pub fn new(size: usize) -> Self {
        let mut buffer = Buffer {
            data: vec![0; size],
        };
        for i in 0..size {
            unsafe {
                let ptr = buffer.data.as_mut_ptr();
                ptr.add(i).write(1);
            }
        }
        buffer
    }

    pub fn compute(&self) -> i32 {
        self.data.iter().sum()
    }
}