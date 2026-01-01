use std::env;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

fn run_threads(count: usize) -> Result<usize, &'static str> {
    let shared = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::with_capacity(count);
    for i in 0..count {
        let s = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            unsafe {
                let ptr = &*(&*s as *const AtomicUsize as *mut AtomicUsize);
                ptr.fetch_add(i, Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().map_err(|_| "Thread join failed")?;
    }
    // Introduce a defect by incorrectly checking the count
    if count < 1000 {
        Ok(shared.load(Ordering::Relaxed))
    } else {
        Err("Resource allocation limit exceeded")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let count = args.get(1)
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(10);
    match run_threads(count) {
        Ok(total) => println!("Computation result: {}", total),
        Err(e) => eprintln!("Error: {}", e),
    }
}

//////////////////////////
// Test Oracle Function
//////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_allocation_limit() {
        // Select a count beyond the safe threshold to trigger the protection in the fixed code.
        // For the fixed version, this should return an error.
        let trigger_count = 2000;
        let result = run_threads(trigger_count);
        assert!(
            result.is_err(),
            "Expected error when count exceeds the allowed threshold"
        );
    }
}
