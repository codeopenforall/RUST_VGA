use std::sync::{Arc, Mutex};
use std::thread;
fn run_app() -> i32 {
    let shared = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..8 {
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                let mut counter = shared_clone.lock().unwrap();
                *counter += 1;
            }
        });
        handles.push(handle);
    }
    for h in handles {
        h.join().unwrap();
    }
    let counter = shared.lock().unwrap();
    *counter
}
fn main() {
    let result = run_app();
    println!("Final counter value: {}", result);
}


#[cfg(test)]
mod tests {
    use super::*;
    // This test oracle function is used externally and must be run
    // separately from the vulnerable and fixed binaries.
    //
    // For a correct (fixed) implementation, the final counter value
    // should be exactly 8000. For the vulnerable implementation, due to
    // unsynchronized access, the final counter value is likely to be less.
    #[test]
    fn test_final_counter() {
        let result = run_app();
        assert_eq!(result, 8000, "Final counter value should be 8000, found {}", result);
    }
}
