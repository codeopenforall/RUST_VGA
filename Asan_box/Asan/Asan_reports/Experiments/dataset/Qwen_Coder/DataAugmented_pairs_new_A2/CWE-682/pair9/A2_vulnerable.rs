use std::thread;
use std::sync::Arc;

struct Processor {
    data: Vec<u32>,
    scale: u32,
}

impl Processor {
    fn compute(&self) -> u32 {
        let sum: u32 = self.data.iter().sum();
        let count = self.data.len() as u32;
        let ptr: *const u32 = &sum;
        let unsafe_sum = unsafe { *ptr };
        let result = unsafe_sum / self.scale * count; // Defect introduced: changed order of operations
        result
    }
}

fn main() {
    let proc_obj = Arc::new(Processor { data: vec![1, 2], scale: 100 });
    let mut threads = vec![];
    for _ in 0..2 {
        let proc_clone = Arc::clone(&proc_obj);
        let handle = thread::spawn(move || {
            let output = proc_clone.compute();
            println!("Computed result: {}", output);
        });
        threads.push(handle);
    }
    for handle in threads {
        handle.join().unwrap();
    }
}

////////////////////// Test Oracle //////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scaled_average() {
        // Create a processor instance with a small dataset that exposes the calculation flaw.
        // With data [1, 2] and scale factor 100:
        // The correct calculation: (1 + 2) * 100 / 2 = 300 / 2 = 150.
        // The vulnerable implementation erroneously calculates: ( (1 + 2) / 2 ) * 100 = (3 / 2) * 100 = 1 * 100 = 100.
        let proc_instance = Processor { data: vec![1, 2], scale: 100 };
        let computed = proc_instance.compute();
        assert_eq!(computed, 150, "Computed scaled average should be 150");
    }
}
