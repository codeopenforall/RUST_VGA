struct Calculator {
    data: Vec<u32>,
}

impl Calculator {
    fn new(data: Vec<u32>) -> Self {
        Calculator { data }
    }

    fn process(&self) -> u32 {
        let len = self.data.len() as u32;
        if len == 0 {
            return 0;
        }
        let sum: u32 = self.data.iter().sum();
        sum / len
    }
}