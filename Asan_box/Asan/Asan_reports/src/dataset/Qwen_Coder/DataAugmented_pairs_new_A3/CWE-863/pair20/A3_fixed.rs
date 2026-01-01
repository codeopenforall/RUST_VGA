use std::sync::{Arc, Mutex};
use std::thread;

const MAX_THREADS: usize = 100;
const MAX_ALLOCATION: usize = 10_000;

pub fn execute(count: usize, allocation: usize) -> usize {
    let safe_count = if count > MAX_THREADS { MAX_THREADS } else { count };
    let safe_alloc = if allocation > MAX_ALLOCATION {
        MAX_ALLOCATION
    } else {
        allocation
    };

    let shared = Arc::new(Mutex::new(vec![]));

    let mut handles = vec![];

    for _ in 0..safe_count {
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            let buf = vec![0u8; safe_alloc];
            let calc = buf.iter().fold(0, |acc, &val| acc + val as usize);
            let mut locked = shared_clone.lock().unwrap();
            locked.push((calc % 256) as u8);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let result = shared.lock().unwrap().iter().fold(0, |acc, &val| acc + val as usize);
    result
}

///////////////////////////////////////////////////////////////
// Test Oracle Function
///////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    // Import the execute function from the library.
    use super::execute;

    // This test simulates a case where an untrusted caller provides excessive
    // resource parameters. The fixed version should throttle the parameters and return 0,
    // while the vulnerable version (if unchecked) may produce undefined behavior or panic.
    #[test]
    fn oracle() {
        // Triggering input with excessive values.
        // In the fixed version, these parameters are capped (100 threads, 10_000 allocation),
        // and since each thread initializes a buffer of zeros, the aggregated sum will be 0.
        let result = execute(150, 20000);
        assert_eq!(result, 0, "The aggregated result should be 0 after safe throttling.");
    }
}
