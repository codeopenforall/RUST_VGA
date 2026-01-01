use std::sync::{Arc, Mutex};
use std::thread;

struct DataProcessor {
    threshold: u32,
    data: Vec<u32>,
}

impl DataProcessor {
    fn new(threshold: u32, data: Vec<u32>) -> Self {
        Self { threshold, data }
    }

    fn calculate(&self, index: u32) -> Option<u32> {
        if index < self.threshold {
            return None;
        }
        self.data.get(index as usize).copied()
    }
}

fn main() {
    let processor = Arc::new(Mutex::new(DataProcessor::new(10, vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110])));
    let mut handles = vec![];
    for i in 0..5 {
        let proc_clone = Arc::clone(&processor);
        handles.push(thread::spawn(move || {
            let res = proc_clone.lock().unwrap().calculate(i);
            println!("Thread {} got: {:?}", i, res);
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    let result = processor.lock().unwrap().calculate(10);
    println!("Main result: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    // The testing contract: for an input below the threshold, the function must return None.
    // In the corrected implementation, this should pass as the input is safely rejected.
    // However, in the vulnerable implementation the unsafe branch may lead to an incorrect value or undefined behavior.
    #[test]
    fn test_control_flow() {
        // Prepare a processor with a threshold of 10, and a data vector with 5 elements.
        let proc_instance = DataProcessor::new(10, vec![10, 20, 30, 40, 50]);
        let result = proc_instance.calculate(5); // 5 is below the threshold.
        assert!(result.is_none(), "Expected None for input below the threshold");
    }
}
