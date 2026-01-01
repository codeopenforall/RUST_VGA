struct Processor {
    inner: Vec<i32>,
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            inner: vec![1, 2, 3, 4],
        }
    }

    pub fn execute(&self, idx: usize) {
        let value = self.inner[idx];
        assert!(value < 3, "Assertion triggered: encountered value {} which is too high", value);
        println!("Main completed in the vulnerable version");
    }
}