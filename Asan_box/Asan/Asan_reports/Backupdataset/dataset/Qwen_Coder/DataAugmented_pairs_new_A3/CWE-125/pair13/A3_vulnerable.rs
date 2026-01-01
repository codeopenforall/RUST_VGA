struct Processor {
    data: Box<[i32]>,
}

impl Processor {
    pub fn compute(&self, start: usize, count: usize) -> i32 {
        unsafe {
            let ptr = self.data.as_ptr().add(start);
            let slice = std::slice::from_raw_parts(ptr, count);
            slice.iter().sum()
        }
    }
}