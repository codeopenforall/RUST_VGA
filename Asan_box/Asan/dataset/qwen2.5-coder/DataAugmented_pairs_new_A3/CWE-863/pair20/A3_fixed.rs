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