use std::sync::{Arc, Barrier};
use std::thread;

struct Processor {
    multiplier: u64,
}

impl Processor {
    unsafe fn compute(&self, value: u64) -> u16 {
        let result = value * self.multiplier;
        result as u16
    }
}

fn run(input: u64) -> u16 {
    let processor = Arc::new(Processor { multiplier: 2 });
    let barrier = Arc::new(Barrier::new(4));
    let mut threads = vec![];
    let mut outcomes = vec![];

    for _ in 0..4 {
        let proc_clone = Arc::clone(&processor);
        let barrier_clone = Arc::clone(&barrier);
        let thread_handle = thread::spawn(move || {
            barrier_clone.wait();
            unsafe { proc_clone.compute(input) }
        });
        threads.push(thread_handle);
    }

    for thread_handle in threads {
        outcomes.push(thread_handle.join().unwrap());
    }

    outcomes.into_iter().sum()
}

fn main() {
    let input = 70000u64;
    let sum = run(input);
    println!("Sum: {}", sum);
}