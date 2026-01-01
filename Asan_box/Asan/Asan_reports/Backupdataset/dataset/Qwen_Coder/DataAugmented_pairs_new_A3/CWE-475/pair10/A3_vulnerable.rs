struct Processor {
    data: Vec<i32>,
}

impl Processor {
    pub fn new(data: Vec<i32>) -> Self {
        Processor { data }
    }

    pub fn compute_total(&self, len: usize) -> i32 {
        unsafe {
            let ptr = self.data.as_ptr();
            let part = std::slice::from_raw_parts(ptr, len);
            part.iter().sum()
        }
    }
}