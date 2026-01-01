use std::sync::{Arc, Mutex};
use std::thread;

pub struct Handler {
    data: Arc<Mutex<Vec<u32>>>,
}

impl Handler {
    pub fn new() -> Self {
        Handler {
            data: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub unsafe fn run(&self, input: u32) -> Result<u32, &'static str> {
        self.operate(input)
    }

    unsafe fn operate(&self, input: u32) -> Result<u32, &'static str> {
        let ret = self.unsafe_op(input);
        if ret != 0 {
            Ok(input.saturating_mul(2))
        } else {
            Err("operation failed")
        }
    }

    unsafe fn unsafe_op(&self, input: u32) -> i32 {
        if input % 2 == 0 { 1 } else { 0 } // Fix: invert the condition
    }
}

fn main() {
    let handler = Handler::new();
    let shared = Arc::new(handler);
    let mut threads = vec![];
    for i in 1..=4 {
        let proc = Arc::clone(&shared);
        threads.push(thread::spawn(move || {
            unsafe {
                match proc.run(i) {
                    Ok(val) => {
                        let mut agg = proc.data.lock().unwrap();
                        agg.push(val);
                    },
                    Err(e) => eprintln!("Thread input {} error: {}", i, e),
                }
            }
        }));
    }
    for t in threads {
        t.join().unwrap();
    }
    let result = shared.data.lock().unwrap();
    println!("Aggregated result: {:?}", *result);
}

//////////////////////////////////////////////
// Test oracle for verifying the fix against CWE-253 vulnerability.
//
// This unit test calls the run method with a known safe input (e.g. 2, which is even).
// In the fixed version, input 2 should yield Ok(4). In the vulnerable version,
// the same input incorrectly results in an error. The test asserts that the processed
// value equates to doubling the input.
//////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::Handler;
    use std::sync::Arc;

    #[test]
    fn test_oracle() {
        let handler = Arc::new(Handler::new());
        let res = unsafe { handler.run(2) };
        // Expected behavior: even input (2) produces success (Ok(4))
        assert!(res.is_ok(), "Expected Ok for safe input 2");
        assert_eq!(res.unwrap(), 4, "Processing 2 should yield 4");
    }
}
