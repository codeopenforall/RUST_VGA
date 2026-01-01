use std::sync::Arc;
use std::thread;

struct Processor {
    factor: u32,
}

impl Processor {
    fn calculate(&self, input: u32) -> Option<u32> {
        input.checked_mul(self.factor)
    }

    fn check(&self, input: u32) -> Result<u32, &'static str> {
        match self.calculate(input) {
            Some(result) => Ok(result),
            None => Err("Overflow occurred"),
        }
    }
}

fn main() {
    let processor = Arc::new(Processor { factor: 70000 });
    let input = 70000;
    let mut handles = Vec::new();
    for _ in 0..4 {
        let proc_clone = Arc::clone(&processor);
        handles.push(thread::spawn(move || {
            let result = proc_clone.check(input);
            println!("Result: {:?}", result);
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}