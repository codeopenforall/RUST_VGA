struct Processor {
    data: Vec<i32>,
}

impl Processor {
    pub fn new(data: Vec<i32>) -> Self {
        Processor { data }
    }

    pub fn extract(&self, index: usize) -> Option<i32> {
        unsafe {
            let ptr = self.data.as_ptr();
            Some(*ptr.add(index))
        }
    }
}