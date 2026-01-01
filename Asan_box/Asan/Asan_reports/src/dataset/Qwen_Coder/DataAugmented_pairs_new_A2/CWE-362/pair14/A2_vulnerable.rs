use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct Shared {
    count: UnsafeCell<u32>,
}

unsafe impl Sync for Shared {}

impl Shared {
    pub fn new() -> Self {
        Shared { count: UnsafeCell::new(0) }
    }

    pub fn process(&self) {
        unsafe {
            if *self.count.get() == 0 {
                thread::sleep(Duration::from_millis(50));
                *self.count.get() += 1;
            }
        }
    }

    pub fn value(&self) -> u32 {
        unsafe { *self.count.get() }
    }
}

fn main() {
    let shared = Arc::new(Shared::new());
    let mut handles = vec![];
    for _ in 0..2 {
        let s = Arc::clone(&shared);
        handles.push(thread::spawn(move || {
            s.process();
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Final count: {}", shared.value());
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Test Oracle: Verifying the Invariant that Exactly One Thread Performs the Update
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    // This test verifies that after concurrently invoking process(), the shared counter 
    // remains exactly 1. In the presence of a race condition, the vulnerable version may 
    // result in a counter value greater than 1.
    #[test]
    fn invariant_test() {
        let shared = Arc::new(Shared::new());
        let mut handles = vec![];

        // Increase concurrency by launching multiple threads.
        for _ in 0..10 {
            let s = Arc::clone(&shared);
            handles.push(std::thread::spawn(move || {
                s.process();
            }));
        }
        for h in handles {
            h.join().unwrap();
        }

        // The following assertion should pass for the fixed code and fail for the vulnerable code.
        assert_eq!(shared.value(), 1, "Invariant violated: more than one update occurred");
    }
}
