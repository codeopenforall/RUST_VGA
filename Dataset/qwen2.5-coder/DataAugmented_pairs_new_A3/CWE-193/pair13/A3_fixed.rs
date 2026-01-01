struct Buffer {
    data: Vec<u32>,
}

impl Buffer {
    pub fn new(n: usize) -> Self {
        let data = vec![0; n];
        Buffer { data }
    }

    pub fn populate(&mut self) {
        let n = self.data.len();
        for i in 0..n {
            self.data[i] = (i + 1) as u32;
        }
    }

    pub fn concurrent_sum(&self) -> u32 {
        self.data.iter().sum()
    }
}