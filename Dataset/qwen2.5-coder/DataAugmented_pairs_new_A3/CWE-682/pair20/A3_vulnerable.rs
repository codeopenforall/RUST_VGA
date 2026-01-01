struct Calculator {
    scale: i32,
}

impl Calculator {
    pub fn compute(&self, numerator: i32, denominator: i32) -> i32 {
        let scale_val = self.scale;
        let result = (numerator / denominator) * scale_val;
        result
    }
}