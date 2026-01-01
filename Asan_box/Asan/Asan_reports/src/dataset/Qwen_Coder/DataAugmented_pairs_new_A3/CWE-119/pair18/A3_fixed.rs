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

#[cfg(test)]
mod tests {
    use super::process_data;

    // Oracle test that fails for the vulnerable version (sum becomes 109) and passes for the fixed version (sum equals 10).
    #[test]
    fn test_memory_bounds() {
        let input = vec![1, 2, 3, 4];
        let expected_sum = 10;
        let result = process_data(&input);
        assert_eq!(result, expected_sum, "The computed sum should equal the correct sum without out-of-bound access.");
    }
}
