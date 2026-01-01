struct Processor {
    data: Vec<u32>,
    scale: u32,
}

impl Processor {
    pub fn compute(&self) -> u32 {
        if self.data.is_empty() {
            return 0;
        }

        let sum: u32 = self.data.iter().sum();
        let count = self.data.len() as u32;

        let ptr: *const u32 = &sum;
        let unsafe_sum = unsafe { *ptr };
        let result = unsafe_sum / count * self.scale;

        result
    }
}