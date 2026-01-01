struct Processor {
    inner: Vec<u8>,
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            inner: vec![1, 2, 3, 4, 5],
        }
    }

    pub fn execute(&self, idx: usize) {
        if idx >= self.inner.len() {
            eprintln!("Input error: index {} out of range", idx);
            return;
        }

        let value = self.inner[idx];
        if value >= 3 {
            eprintln!("Rejected input: value {} is too high", value);
            return;
        }

        println!("Main completed in the corrected version");
    }
}