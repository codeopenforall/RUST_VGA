use std::sync::{Arc, Mutex};
use std::thread;
pub struct Processor {}
impl Processor {
    unsafe fn compute(&self, input: i32) -> Result<i32, &'static str> {
        let ptr = &input as *const i32;
        let val = *ptr;
        if val < 0 {
            return Err("Negative input not allowed");
        }
        Ok(val * 2)
    }
    pub fn process(&self, input: i32) -> Result<i32, &'static str> {
        unsafe {
            let outcome = self.compute(input);
            if outcome.is_err() {
                return Ok(0); // Vulnerability introduced here
            }
            outcome
        }
    }
}
fn main() {
    let processor = Arc::new(Processor {});
    let results = Arc::new(Mutex::new(Vec::new()));
    let handles: Vec<_> = (0..4)
        .map(|i| {
            let proc_clone = Arc::clone(&processor);
            let res_clone = Arc::clone(&results);
            thread::spawn(move || {
                let input = if i % 2 == 0 { 10 } else { -5 };
                let outcome = proc_clone.process(input);
                let mut vec = res_clone.lock().unwrap();
                vec.push(match outcome {
                    Ok(val) => val,
                    Err(_) => -1,
                });
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    let res = results.lock().unwrap();
    println!("Processing outputs: {:?}", *res);
}

//////////////////////////////////////////////
// Test Oracle for Verifying Correctness
//////////////////////////////////////////////
#[cfg(test)]
mod test_oracle {
    use super::Processor;

    // This helper function instantiates the Processor and processes the given input,
    // returning the Result as-is.
    fn process_input(input: i32) -> Result<i32, &'static str> {
        let proc_inst = Processor {};
        proc_inst.process(input)
    }

    #[test]
    fn oracle_test() {
        // For a negative input, the correct (fixed) implementation should return an Err.
        // The vulnerable implementation, however, erroneously returns Ok(0).
        let negative_input = -5;
        let res = process_input(negative_input);
        match res {
            Ok(val) => {
                // If the function returns Ok, then the vulnerability is present.
                panic!("Test failed: Negative input produced Ok({}) instead of an error.", val);
            },
            Err(msg) => {
                // The fixed version should reach here.
                assert_eq!(msg, "Negative input not allowed", "Unexpected error message");
            },
        }
    }
}
