use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::thread;

pub fn run() -> u32 {
    let counter = Arc::new(AtomicU32::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    counter.load(Ordering::SeqCst)
}