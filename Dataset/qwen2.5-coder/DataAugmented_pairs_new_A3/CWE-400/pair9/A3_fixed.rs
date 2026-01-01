struct Worker;
impl Worker {
    pub fn execute(&self, data: Vec<u32>) -> u32 {
        data.iter().sum()
    }
}

pub fn run_fn(data: Vec<u32>) -> u32 {
    let worker = Worker;
    worker.execute(data)
}