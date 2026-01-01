struct Calculator {
    data: Vec<u32>,
}

impl Calculator {
    pub fn new(data: Vec<u32>) -> Self {
        Calculator { data }
    }

    pub fn process(&self) -> u32 {
        let len = self.data.len() as u32;
        if len == 0 {
            return 0;
        }
        let mut result: u32 = 0;
        unsafe {
            let ptr = self.data.as_ptr();
            for i in 0..self.data.len() {
                result += *ptr.add(i) / len;
            }
        }
        result
    }
}