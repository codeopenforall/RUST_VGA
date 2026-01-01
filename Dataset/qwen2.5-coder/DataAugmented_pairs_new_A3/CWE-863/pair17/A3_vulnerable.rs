use std::sync::{Arc, atomic::{AtomicU64, Ordering}};
use std::thread;

pub fn execute(n: usize) -> Result<u64, &'static str> {
    let total = Arc::new(AtomicU64::new(0));
    let mut handles = vec![];

    unsafe {
        for i in 0..n {
            let total_clone = Arc::clone(&total);
            let h = thread::spawn(move || {
                total_clone.fetch_add(i as u64, Ordering::SeqCst);
            });
            handles.push(h);
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(total.load(Ordering::SeqCst))
}