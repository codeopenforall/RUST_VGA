use std::env;
use std::sync::Arc;
use std::thread;

struct Executor {
    data: Arc<Vec<u8>>,
}

impl Executor {
    fn new() -> Self {
        Executor {
            data: Arc::new(vec![1, 2, 3, 4, 5]),
        }
    }

    fn run(&self, position: usize) {
        unsafe {
            let pointer = self.data.as_ptr();
            let element = *pointer.add(position);
            assert!(element < 3, "Assertion failed: value {} exceeds limit", element);
            println!("Element processed: {}", element);
        }
    }
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let index: usize = if arguments.len() > 1 {
        arguments[1].parse().unwrap_or(0)
    } else {
        0
    };
    let executor_instance = Executor::new();
    let shared_data = executor_instance.data.clone();
    let thread_handle = thread::spawn(move || {
        let local_executor = Executor { data: shared_data };
        local_executor.run(index);
    });
    thread_handle.join().unwrap();
    println!("Main execution completed in the vulnerable version");
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
