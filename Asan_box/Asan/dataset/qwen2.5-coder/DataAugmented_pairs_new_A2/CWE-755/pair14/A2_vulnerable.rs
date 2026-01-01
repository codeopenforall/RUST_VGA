use std::sync::Arc;
use std::thread;

struct Processor {
    factor: i32,
}

impl Processor {
    fn initialize(factor: i32) -> Self {
        Processor { factor }
    }

    fn compute(&self, divisor_str: &str) -> i32 {
        let divisor: i32 = divisor_str.parse().unwrap_or_default();
        unsafe {
            let ptr = &self.factor as *const i32;
            let value = *ptr;
            value / divisor
        }
    }
}

pub fn execute(divisor_str: &str) -> Result<i32, String> {
    let processor = Processor::initialize(100);
    Ok(processor.compute(divisor_str))
}

fn main() {
    let processor = Arc::new(Processor::initialize(100));
    let arg = std::env::args().nth(1).unwrap_or_default();
    let mut threads = Vec::new();
    for _ in 0..4 {
        let processor = Arc::clone(&processor);
        let arg_clone = arg.clone();
        threads.push(thread::spawn(move || {
            let res = processor.compute(&arg_clone);
            println!("Result: {}", res);
        }));
    }
    for thread in threads {
        thread.join().expect("Thread panicked");
    }
}