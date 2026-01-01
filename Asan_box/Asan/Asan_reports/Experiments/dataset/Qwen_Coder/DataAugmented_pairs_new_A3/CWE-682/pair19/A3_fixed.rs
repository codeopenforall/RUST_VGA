#![allow(unused)]
use std::sync::{Arc, Mutex};
use std::thread;

struct TimeProcessor {
    unit: u64,
}

impl TimeProcessor {
    fn new() -> Self {
        TimeProcessor { unit: 1000 }
    }

    fn compute(&self, secs: u64) -> u64 {
        secs * self.unit
    }
}

fn main() {
    let processor = Arc::new(Mutex::new(TimeProcessor::new()));
    let proc_clone = Arc::clone(&processor);
    let handle = thread::spawn(move || {
        let mut proc = proc_clone.lock().unwrap();
        proc.compute(500)
    });
    let thread_result = handle.join().unwrap();
    println!("Thread computation result: {}", thread_result);
    let direct_result = processor.lock().unwrap().compute(1500);
    println!("Direct computation result: {}", direct_result);
}

//////////////////////////////////////////////////////////////
// Test Oracle Function for Conversion Calculation            //
// This test verifies that the conversion function returns       //
// the expected value. For an input of 500 seconds, the correct    //
// conversion should yield 500,000 milliseconds.                  //
//////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn oracle_test() {
        let processor = TimeProcessor::new();
        // For a 500-second input, the correct computation is 500 * 1000 = 500,000.
        let expected = 500_000;
        let actual = processor.compute(500);
        assert_eq!(actual, expected, "Conversion calculation is incorrect.");
    }
}
