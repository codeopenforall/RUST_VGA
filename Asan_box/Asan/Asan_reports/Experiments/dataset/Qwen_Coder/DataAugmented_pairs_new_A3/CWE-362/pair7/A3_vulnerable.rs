use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;

struct Data {
    value: UnsafeCell<u32>,
}

unsafe impl Sync for Data {}

impl Data {
    fn increment(&self) -> u32 {
        unsafe {
            let ptr = self.value.get();
            let curr = *ptr;
            thread::yield_now();
            *ptr = curr + 1;
        }
        unsafe { *self.value.get() }
    }
}

pub fn run() -> u32 {
    let shared = Arc::new(Data { value: UnsafeCell::new(0) });
    let mut handles = vec![];

    for _ in 0..10 {
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                shared_clone.increment();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    unsafe { *shared.value.get() }
}

///////////////////////////////
// Test Oracle Function
///////////////////////////////
#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_final_count() {
        // The expected correct final count is 10 threads * 1000 increments = 10000.
        // In the vulnerable version, due to the race condition the final count is likely to be less than 10000.
        // In the corrected version, the final count should reliably be 10000.
        let result = run();
        assert_eq!(result, 10000, "The final count should be 10000");
    }
}
