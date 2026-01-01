#![allow(unused)]
use std::sync::{Arc, Mutex};
use std::thread;
struct BufferProcessor {
    data: Vec<u32>,
}
impl BufferProcessor {
    fn new(capacity: usize) -> Self {
        BufferProcessor {
            data: Vec::with_capacity(capacity),
        }
    }
    fn load_data(&mut self, src: &[u32]) {
        unsafe {
            self.data.set_len(src.len());
            let dst_ptr = self.data.as_mut_ptr();
            for i in 0..src.len() {
                *dst_ptr.add(i) = *src.get_unchecked(i + 1); // Off-by-one error
            }
        }
    }
    fn process(&mut self, src: &[u32]) -> u32 {
        self.load_data(src);
        self.data.iter().sum()
    }
}
fn main() {
    let input = vec![10, 20, 30, 40];
    let mut processor = BufferProcessor::new(input.len());
    let result = processor.process(&input);
    println!("Result: {}", result);
    let sum = run_concurrent();
    println!("Concurrent Total: {}", sum);
}
fn run_concurrent() -> u32 {
    let input = vec![10, 20, 30, 40];
    let processor = Arc::new(Mutex::new(BufferProcessor::new(input.len())));
    let input_arc = Arc::new(input);
    let mut handles = Vec::new();
    for _ in 0..2 {
        let proc_clone = Arc::clone(&processor);
        let in_clone = Arc::clone(&input_arc);
        let handle = thread::spawn(move || {
            let mut proc = proc_clone.lock().expect("Lock poisoned");
            proc.process(&in_clone)
        });
        handles.push(handle);
    }
    let mut total = 0;
    for handle in handles {
        total += handle.join().expect("Thread panicked");
    }
    total
}

#[cfg(test)]
mod tests {
    use super::run_concurrent;

    #[test]
    fn test_processing() {
        // The given input vector [10, 20, 30, 40] should produce a sum of 100 per processing instance.
        // Running concurrently (two threads) should result in a total of 200.
        // In the vulnerable version, the off-by-one error causes undefined behavior (panic or incorrect sum),
        // while the fixed version reliably returns 200.
        let result = run_concurrent();
        assert_eq!(result, 200, "Concurrent processing sum mismatch");
    }
}
