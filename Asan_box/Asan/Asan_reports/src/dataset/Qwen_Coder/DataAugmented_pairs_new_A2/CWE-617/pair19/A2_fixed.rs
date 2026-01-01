use std::env;
use std::sync::Arc;
use std::thread;

struct Processor {
    inner: Arc<Vec<u8>>,
}

impl Processor {
    fn new() -> Self {
        Processor {
            inner: Arc::new(vec![1, 2, 3, 4, 5]),
        }
    }

    fn execute(&self, idx: usize) {
        if idx >= self.inner.len() {
            eprintln!("Input error: index {} out of range", idx);
            return;
        }
        unsafe {
            let ptr = self.inner.as_ptr();
            let value = *ptr.add(idx);
            if value >= 3 {
                eprintln!("Rejected input: value {} is too high", value);
                return;
            }
            println!("Processed value: {}", value);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let idx: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(0)
    } else {
        0
    };
    let proc_obj = Processor::new();
    let shared = proc_obj.inner.clone();
    let handler = thread::spawn(move || {
        let local_proc = Processor { inner: shared };
        local_proc.execute(idx);
    });
    handler.join().unwrap();
    println!("Main completed in the corrected version");
}

/* Test oracle function for verifying the behavior.
   This test calls the execute() method using an index (2) that corresponds to a buffer value of 3.
   - In the vulnerable version, this will trigger a panic due to the assertion, causing the test to fail.
   - In the corrected version, the input is gracefully handled without panicking, causing the test to pass.
   The test is deterministic and does not rely on timing assumptions.
*/
#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn test_oracle() {
        let processor = Processor::new();
        // For the vulnerable version, calling execute(2) will panic.
        // Our oracle expects a successful run (i.e. no panic) for the corrected version.
        let result = panic::catch_unwind(|| {
            processor.execute(2);
        });
        // The test fails (i.e. returns Err) in the vulnerable case, and passes (i.e. returns Ok) in the corrected case.
        assert!(result.is_ok(), "The process should not panic for index 2 in the corrected version");
    }
}
