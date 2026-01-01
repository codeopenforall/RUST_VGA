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
                let current = counter_clone.load(Ordering::SeqCst);
                thread::yield_now(); // Introducing a yield to simulate a race condition
                counter_clone.store(current + 1, Ordering::SeqCst);
            }
        }));
    }
    for handle in handles {
        let _ = handle.join();
    }
    counter.load(Ordering::SeqCst)
}
fn main() {
    let final_count = run();
    println!("Final counter value: {}", final_count);
}