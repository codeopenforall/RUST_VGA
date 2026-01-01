use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Shared {
    counter: UnsafeCell<u32>,
}

unsafe impl Sync for Shared {}

fn run_app() -> u32 {
    let shared = Arc::new(Shared {
        counter: UnsafeCell::new(0),
    });

    let mut handles = vec![];

    for _ in 0..10 {
        let shared_clone = Arc::clone(&shared);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                unsafe {
                    let current = *shared_clone.counter.get();
                    if current < 10000 {
                        thread::sleep(Duration::from_micros(1));
                        *shared_clone.counter.get() = current + 1;
                    }
                }
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    unsafe { *shared.counter.get() }
}