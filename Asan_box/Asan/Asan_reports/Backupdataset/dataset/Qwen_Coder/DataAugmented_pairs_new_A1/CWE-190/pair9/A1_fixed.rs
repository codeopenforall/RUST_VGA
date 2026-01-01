use std::sync::{Arc, Mutex};
use std::thread;
use std::num::Wrapping;

struct Accumulator {
    balance: Wrapping<u32>,
}

impl Accumulator {
    fn new(init: u32) -> Self {
        Accumulator { balance: Wrapping(init) }
    }
    fn add(&mut self, amount: u32) {
        self.balance += Wrapping(amount);
    }
    fn current(&self) -> u32 {
        self.balance.0
    }
}

fn run_calculation(init: u32, add: u32, threads: usize) -> Result<u32, &'static str> {
    let accumulator = Arc::new(Mutex::new(Accumulator::new(init)));
    let mut handles = vec![];
    for _ in 0..threads {
        let acc_clone = Arc::clone(&accumulator);
        handles.push(thread::spawn(move || {
            let mut guard = acc_clone.lock().unwrap();
            guard.add(add);
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let guard = accumulator.lock().unwrap();
    if guard.balance.0 < init {
        Err("Overflow occurred")
    } else {
        Ok(guard.current())
    }
}

pub fn try_compute() -> Result<u32, &'static str> {
    let result = run_calculation(u32::MAX - 10, 15, 1)?;
    Ok(result)
}

fn main() {
    let result = try_compute().unwrap();
    println!("Final result: {}", result);
}