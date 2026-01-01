struct Processor {
    data: std::sync::Arc<u32>,
}

impl Processor {
    pub fn new(val: u32) -> Self {
        Processor {
            data: std::sync::Arc::new(val),
        }
    }

    pub fn compute(&self) -> u32 {
        *self.data
    }
}

pub fn execute() -> u32 {
    let job = Processor::new(42);
    job.compute()
}