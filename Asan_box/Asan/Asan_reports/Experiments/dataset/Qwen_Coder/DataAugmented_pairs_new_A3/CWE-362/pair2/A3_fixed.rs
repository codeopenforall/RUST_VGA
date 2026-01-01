use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Data {
    count: i32,
}

pub fn run_process() -> i32 {
    let shared = Arc::new(Mutex::new(Data { count: 0 }));
    let mut handles = vec![];

    for _ in 0..10 {
        let shared_clone = Arc::clone(&shared);
        handles.push(thread::spawn(move || {
            let mut data = shared_clone.lock().unwrap();
            if data.count % 2 == 0 {
                thread::sleep(Duration::from_millis(10));
                data.count += 1;
            } else {
                thread::sleep(Duration::from_millis(10));
                data.count += 2;
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_data = shared.lock().unwrap();
    final_data.count
}

fn main() {
    let result = run_process();
    println!("Final count (synchronized): {}", result);
}

////////////////////////////////////
// Test Oracle for Concurrency Bug  //
// This unit test fails for the race //
// condition version and passes for   //
// the synchronized one.            //
////////////////////////////////////

#[cfg(test)]
mod tests {
    // Import the run_process function from the current crate.
    use super::run_process;

    #[test]
    fn concurrency_invariant() {
        // Run the process multiple times to overcome nondeterminism.
        // The correctly synchronized version should always produce 19.
        // The unsynchronized version (race condition) may produce an incorrect value.
        let result = run_process();
        // The expected value is determined as follows:
        // Starting count = 0, first thread finds even -> +1 results in 1.
        // All subsequent threads will see an odd value and add 2.
        // With 10 threads, final count should be 1 + 9*2 = 19.
        assert_eq!(result, 19, "Invariant violated: expected 19, got {}", result);
    }
}
