use std::sync::Arc;
use std::thread;

struct SafeLocker {
    data: Arc<Vec<u64>>,
}

impl SafeLocker {
    fn new() -> Self {
        SafeLocker { data: Arc::new(vec![0; 10]) }
    }

    fn update(&self, index: usize, value: u64) {
        unsafe {
            let ptr = self.data.as_ptr() as *mut u64;
            *ptr.add(index) = value;
            // Introducing a CWE-7/pair5 defect: Removing the bounds check
            // assert!(index < self.data.len(), "Index out of range: {}", index);
        }
    }
}

fn main() {
    let locker = SafeLocker::new();
    let locker_clone = locker;
    let handle = thread::spawn(move || {
        locker_clone.update(15, 42);
    });
    let _ = handle.join();
    println!("Execution completed.");
}