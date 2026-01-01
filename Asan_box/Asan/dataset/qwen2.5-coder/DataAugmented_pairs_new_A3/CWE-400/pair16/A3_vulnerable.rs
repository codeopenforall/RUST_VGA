use std::sync::{Arc, atomic::{AtomicU64, Ordering}};
use std::thread;

pub fn process(limit: u32) -> Result<(), &'static str> {
    let total = Arc::new(AtomicU64::new(0));
    let mut handles = vec![];

    for i in 0..limit {
        unsafe {
            let handle = thread::spawn(move || {
                let mut local = i as u64;
                let ptr = &mut local as *mut u64;
                *ptr = *ptr + 1;
                total.fetch_add(*ptr, Ordering::Relaxed);
            });
            handles.push(handle);
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}