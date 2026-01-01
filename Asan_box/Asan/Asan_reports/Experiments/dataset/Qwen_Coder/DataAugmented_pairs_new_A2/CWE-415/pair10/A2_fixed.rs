use std::sync::atomic::{AtomicUsize, Ordering};
use std::ptr;
static GLOBAL_COUNT: AtomicUsize = AtomicUsize::new(0);
struct Handler {
    raw: *mut i32,
}
impl Handler {
    unsafe fn create(num: i32) -> Self {
        let boxed = Box::new(num);
        Self {
            raw: Box::into_raw(boxed),
        }
    }
    unsafe fn dispose(&mut self) {
        if !self.raw.is_null() {
            if GLOBAL_COUNT.fetch_add(1, Ordering::SeqCst) >= 1 {
                panic!("Double free detected in dispose");
            }
            let _ = Box::from_raw(self.raw);
            self.raw = ptr::null_mut(); 
        }
    }
}
impl Drop for Handler {
    fn drop(&mut self) {
        unsafe {
            if !self.raw.is_null() {
                if GLOBAL_COUNT.fetch_add(1, Ordering::SeqCst) >= 1 {
                    panic!("Double free detected in Drop");
                }
                let _ = Box::from_raw(self.raw);
                self.raw = ptr::null_mut(); // Ensure raw is set to null after dropping
            }
        }
    }
}
fn main() {
    unsafe {
        let mut obj = Handler::create(200);
        obj.dispose(); 
    }
    println!("Execution completed (corrected path).");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;
    use std::sync::atomic::Ordering;

    // This test oracle function expects the safe implementation to execute
    // without causing a panic. For the vulnerable path, the double free will
    // trigger a panic, causing the test to fail.
    #[test]
    fn test_resource_management() {
        // Reset the global counter to ensure a clean test environment.
        GLOBAL_COUNT.store(0, Ordering::SeqCst);

        // The test will catch any panics during the resource's lifetime.
        let result = panic::catch_unwind(|| {
            unsafe {
                let mut instance = Handler::create(123);
                instance.dispose();
                // On drop, if the pointer is not null, deallocation is attempted.
            }
        });
        // Oracle expects no panic (i.e. no double free) for the corrected version.
        assert!(result.is_ok(), "Oracle: Expected resource to be safely managed (no double free)");
    }
}
