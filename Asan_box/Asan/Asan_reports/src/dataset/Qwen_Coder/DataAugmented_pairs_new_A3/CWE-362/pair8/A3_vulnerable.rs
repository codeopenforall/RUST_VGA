use std::{cell::UnsafeCell, sync::Arc, thread, time::Duration};

pub struct Resource {
    flag: UnsafeCell<bool>,
    init_count: UnsafeCell<u32>,
}

unsafe impl Sync for Resource {}

impl Resource {
    pub fn new() -> Self {
        Resource {
            flag: UnsafeCell::new(false),
            init_count: UnsafeCell::new(0),
        }
    }

    pub fn process(&self) {
        unsafe {
            if !*self.flag.get() {
                thread::sleep(Duration::from_millis(10));
                *self.init_count.get() = *self.init_count.get() + 1;
                *self.flag.get() = true;
            }
        }
    }

    pub fn get_count(&self) -> u32 {
        unsafe { *self.init_count.get() }
    }
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
