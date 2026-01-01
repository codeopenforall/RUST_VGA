use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;

struct SharedData {
    counter: UnsafeCell<u32>,
}

unsafe impl Sync for SharedData {}

impl SharedData {
    fn update(&self) {
        unsafe {
            let ptr = self.counter.get();
            let current = *ptr;
            thread::yield_now();
            *ptr = current + 1;
        }
    }

    fn fetch(&self) -> u32 {
        unsafe { *self.counter.get() }
    }
}

fn execute() -> u32 {
    let data = Arc::new(SharedData { counter: UnsafeCell::new(0) });
    let mut handles = Vec::new();

    for _ in 0..10 {
        let cloned_data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                cloned_data.update();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    data.fetch()
}

fn main() {
    let final_result = execute();
    println!("Final count: {}", final_result);
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
