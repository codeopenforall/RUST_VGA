use std::{sync::{Arc, Mutex}, thread};

struct Data {
    count: u32,
}

fn run_system() -> u32 {
    let shared = Arc::new(Mutex::new(Data { count: 0 }));
    let mut threads = vec![];
    for _ in 0..10 {
        let shared_clone = Arc::clone(&shared);
        threads.push(thread::spawn(move || {
            for _ in 0..100 {
                let mut data = shared_clone.lock().unwrap();
                data.count += 1;
            }
        }));
    }
    for t in threads {
        t.join().unwrap();
    }
    let result = shared.lock().unwrap().count;
    result
}

fn main() {
    let total = run_system();
    println!("Final count: {}", total);
}

//////////////////////////////// Test Oracle //////////////////////////////////
#[cfg(test)]
mod tests {
    // Import run_system from the current module. When linking against the vulnerable build, this test should fail,
    // while linking against the fixed build, the test should pass.
    use super::run_system;

    #[test]
    fn oracle_test() {
        // The expected final count should be 1000 (10 threads * 100 increments each).
        let result = run_system();
        assert_eq!(result, 1000, "Final count did not match expected value! Race condition exists.");
    }
}
