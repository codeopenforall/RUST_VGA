struct Processor;
impl Processor {
    pub fn run(&self, data: Vec<u32>) -> u32 {
        unsafe {
            let ptr = data.as_ptr();
            let len = data.len();
            let limit = if len > 100 { 100 } else { len };
            let mut total = 0;
            for i in 0..limit {
                total += *ptr.add(i);
            }
            total
        }
    }
}

pub fn run_fn(data: Vec<u32>) -> u32 {
    let proc = Processor;
    proc.run(data)
}