pub struct Aggregator {
    values: Vec<i32>,
}

impl Aggregator {
    pub fn new(values: Vec<i32>) -> Self {
        Aggregator { values }
    }

    pub fn calculate(&self) -> i32 {
        let total: i32 = self.values.iter().sum();
        let len = self.values.len();
        if len == 0 {
            return 0;
        }
        (total / (len as i32)) * 100
    }
}