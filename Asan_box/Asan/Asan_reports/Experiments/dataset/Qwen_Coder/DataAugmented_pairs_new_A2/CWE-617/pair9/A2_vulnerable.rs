use std::sync::Arc;
use std::thread;

struct Processor {
    data: Box<[u32]>,
}

impl Processor {
    fn new(size: usize) -> Self {
        let mut v = Vec::with_capacity(size);
        for i in 0..size {
            v.push(i as u32);
        }
        Self { data: v.into_boxed_slice() }
    }

    unsafe fn fetch(&self, index: usize) -> u32 {
        *self.data.as_ptr().add(index)
    }

    fn process(&self, index: usize, limit: u32) -> u32 {
        unsafe {
            let val = self.fetch(index);
            assert!(val <= limit, "Invariant violation: value {} is not less than or equal to {}", val, limit);
            val
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <index> <threshold>", args[0]);
        std::process::exit(1);
    }
    let idx: usize = args[1].parse().unwrap();
    let threshold: u32 = args[2].parse().unwrap();
    let proc_instance = Arc::new(Processor::new(10));
    let mut threads = Vec::new();
    for _ in 0..4 {
        let proc_clone = Arc::clone(&proc_instance);
        let idx_clone = idx;
        let thr_clone = threshold;
        threads.push(thread::spawn(move || {
            let res = proc_clone.process(idx_clone, thr_clone);
            println!("Result: {}", res);
        }));
    }
    for handle in threads {
        let _ = handle.join();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // This test oracle uses an input that causes the vulnerable code to panic
    // while the fixed code—ignoring the external threshold—succeeds.
    #[test]
    fn contract_test() {
        let instance = Processor::new(10);
        // With index 5, the data value is 5.
        // In the vulnerable version, providing threshold '5' triggers the assertion (5 < 5 fails).
        // In the fixed version, the external threshold is ignored, and the safe internal limit (10) is used.
        let value = instance.process(5, 5);
        assert_eq!(value, 5, "Processing did not return the expected value.");
    }
}
