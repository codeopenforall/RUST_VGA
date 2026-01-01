struct Core {
    size: usize,
}

impl Core {
    pub fn new(size: usize) -> Self {
        Core { size }
    }

    pub fn generate(&self) -> usize {
        let mut sum = 0;
        for index in 0..self.size {
            sum += index;
        }
        sum
    }
}