pub struct DataProcessor {
    data: Vec<i32>,
}

impl DataProcessor {
    pub fn new(data: Vec<i32>) -> Self {
        DataProcessor { data }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn process_data(&self) -> i32 {
        let mut sum = 0;
        for i in 0..self.len() {
            sum += self.data[i];
        }
        sum
    }
}

pub fn process_data(data: &[i32]) -> i32 {
    let processor = DataProcessor::new(data.to_vec());
    processor.process_data()
}