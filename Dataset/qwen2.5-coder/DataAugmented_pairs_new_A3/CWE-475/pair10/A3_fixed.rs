struct Processor {
    data: Vec<i32>,
}

impl Processor {
    pub fn new(data: Vec<i32>) -> Self {
        Processor { data }
    }

    pub fn compute_total(&self, len: usize) -> i32 {
        use std::cmp;
        let valid_len = cmp::min(len, self.data.len());
        self.data[..valid_len].iter().sum()
    }
}