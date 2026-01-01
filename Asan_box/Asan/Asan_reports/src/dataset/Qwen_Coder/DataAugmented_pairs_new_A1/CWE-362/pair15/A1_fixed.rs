use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

struct Counter {
    count: u64,
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }
}

fn run_concurrent(increments: u64, threads: usize) -> u64 {
    let counter = Arc::new(Mutex::new(Counter::new()));
    let mut handles = Vec::with_capacity(threads);

    for _ in 0..threads {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..increments {
                let mut c = counter_clone.lock().unwrap();
                c.count = c.count.wrapping_add(1);
                thread::sleep(Duration::from_micros(1));
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let result = {
        let c = counter.lock().unwrap();
        c.count
    };

    result
}

fn main() {
    let threads = 4;
    let increments = 10000;
    let final_value = run_concurrent(increments, threads);
    println!("Final count: {}", final_value);
}

#[cfg(test)]
mod tests {
    // Import the run_concurrent function from the outer scope.
    use super::*;

    #[test]
    fn test_concurrent() {
        // These parameters are chosen such that the expected final count is:
        // threads * increments = 4 * 10000 = 40000.
        let threads = 4;
        let increments = 10000;
        let result = run_concurrent(increments, threads);
        assert_eq!(result, threads as u64 * increments, 
            "Test failed: The counter did not reach the expected value. Likely a race condition is present.");
    }
}
