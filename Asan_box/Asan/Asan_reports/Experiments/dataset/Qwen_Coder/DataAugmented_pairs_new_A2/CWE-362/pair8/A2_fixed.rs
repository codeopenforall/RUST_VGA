use std::{sync::atomic::{AtomicBool, AtomicU32, Ordering}, sync::Arc, thread, time::Duration};

struct Resource {
    flag: AtomicBool,
    init_count: AtomicU32,
}

impl Resource {
    fn new() -> Self {
        Resource {
            flag: AtomicBool::new(false),
            init_count: AtomicU32::new(0),
        }
    }

    fn process(&self) {
        if !self.flag.load(Ordering::Acquire) {
            thread::sleep(Duration::from_millis(10));
            if self.flag.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                self.init_count.fetch_add(1, Ordering::AcqRel);
            }
        }
    }

    fn get_count(&self) -> u32 {
        self.init_count.load(Ordering::Relaxed)
    }
}

fn main() {
    let res = Arc::new(Resource::new());
    let mut handles = Vec::new();
    for _ in 0..10 {
        let resource = Arc::clone(&res);
        handles.push(thread::spawn(move || {
            for _ in 0..5 {
                resource.process();
            }
        }));
    }
    for handle in handles {
        let _ = handle.join();
    }
    println!("Initialization count: {}", res.get_count());
}

#[cfg(test)]
mod tests {
    use std::{sync::Arc, thread, time::Duration};

    // Assume that the tested module provides a structure named Resource with methods new, process, and get_count.
    use super::Resource;

    #[test]
    fn test_initialization() {
        // Create the shared resource.
        let resource = Arc::new(Resource::new());
        let mut handles = Vec::new();

        // Spawn multiple threads to trigger the initialization.
        for _ in 0..10 {
            let shared = Arc::clone(&resource);
            handles.push(thread::spawn(move || {
                for _ in 0..5 {
                    shared.process();
                    // Optional: simulate additional work.
                    thread::sleep(Duration::from_millis(1));
                }
            }));
        }

        // Wait for all threads to complete.
        for handle in handles {
            handle.join().expect("Thread panicked");
        }

        // Assert that initialization occurred exactly once.
        let count = resource.get_count();
        assert_eq!(count, 1, "Race condition error: initialization count is {}, expected 1", count);
    }
}
