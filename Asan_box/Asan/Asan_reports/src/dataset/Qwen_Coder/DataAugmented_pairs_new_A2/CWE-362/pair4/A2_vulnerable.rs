use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Data {
    value: UnsafeCell<u32>,
}

unsafe impl Sync for Data {}

impl Data {
    fn new(val: u32) -> Self {
        Data {
            value: UnsafeCell::new(val),
        }
    }

    fn update(&self) {
        unsafe {
            let ptr = self.value.get();
            let current = ptr.read();
            thread::sleep(Duration::from_micros(1)); // Introduce a delay to increase race condition likelihood
            ptr.write(current + 1);
        }
    }

    fn fetch(&self) -> u32 {
        unsafe { self.value.get().read() }
    }
}

fn main() {
    let shared = Arc::new(Data::new(0));
    let mut workers = Vec::new();

    for _ in 0..10 {
        let handler = {
            let local = Arc::clone(&shared);
            thread::spawn(move || {
                for _ in 0..1000 {
                    local.update();
                }
            })
        };
        workers.push(handler);
    }

    for worker in workers {
        worker.join().expect("Thread panicked");
    }

    println!("Final value: {}", shared.fetch());
}

//////////////////////////////
// Test Oracle Function
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    // Test to verify that the counter reaches the expected final value.
    // For the vulnerable version this test is expected to fail intermittently 
    // due to the race condition, while the fixed version reliably passes.
    #[test]
    fn check_counter() {
        let shared = Arc::new(Data::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let local = Arc::clone(&shared);
            let handle = thread::spawn(move || {
                for _ in 0..1000 {
                    local.update();
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
        let final_value = shared.fetch();
        // The expected value after 10 threads each incrementing 1000 times.
        assert_eq!(final_value, 10000, "Detected race condition: final value is {}, expected 10000", final_value);
    }
}
