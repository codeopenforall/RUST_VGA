#![allow(unused)]
use std::sync::Arc;
use std::thread;

struct Processor {
    factor: u32,
}

impl Processor {
    unsafe fn calculate(&self, value: u32) -> u64 {
        let result = value.wrapping_mul(self.factor);
        result as u64
    }
}

fn main() {
    let proc = Arc::new(Processor { factor: 100_000 });
    let test_value: u32 = 50_000;
    let proc_clone = Arc::clone(&proc);
    let handle = thread::spawn(move || {
        unsafe { proc_clone.calculate(test_value) }
    });
    let outcome = handle.join().unwrap();
    println!("Calculated outcome: {}", outcome);
    if outcome != 5_000_000_000 {
        std::process::exit(1);
    }
    std::process::exit(0);
}