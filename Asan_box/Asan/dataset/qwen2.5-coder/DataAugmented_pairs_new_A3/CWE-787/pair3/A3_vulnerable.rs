pub struct Buffer {
    data: Vec<i32>,
}

impl Buffer {
    pub fn new(size: usize) -> Self {
        let mut data = vec![0; size];
        let ptr = data.as_mut_ptr();
        for i in 0..size {
            // Intentionally writing to the next element, causing an off-by-one error
            unsafe { ptr.add(i + 1).write(1); }
        }
        Buffer { data }
    }

    pub fn compute(&self) -> i32 {
        self.data.iter().sum()
    }
}