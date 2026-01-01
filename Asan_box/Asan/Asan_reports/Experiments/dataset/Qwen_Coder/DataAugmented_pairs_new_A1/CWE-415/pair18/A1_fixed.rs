use std::sync::atomic::{AtomicUsize, Ordering};
static GLOBAL_FREE_COUNT: AtomicUsize = AtomicUsize::new(0);
pub struct Resource {
    ptr: *mut i32,
}
unsafe impl Send for Resource {}
impl Resource {
    pub fn new(val: i32) -> Self {
        let boxed = Box::new(val);
        let ptr = Box::into_raw(boxed);
        Self { ptr }
    }
    pub unsafe fn manual_release(&mut self) {
        if !self.ptr.is_null() {
            let _ = Box::from_raw(self.ptr);
            GLOBAL_FREE_COUNT.fetch_add(1, Ordering::SeqCst);
            self.ptr = std::ptr::null_mut(); // Prevent double free
        }
    }
    pub unsafe fn get_value(&self) -> i32 {
        *self.ptr
    }
}
impl Drop for Resource {
    fn drop(&mut self) {
        unsafe {
            if !self.ptr.is_null() {
                let _ = Box::from_raw(self.ptr);
                GLOBAL_FREE_COUNT.fetch_add(1, Ordering::SeqCst);
            }
        }
    }
}
pub fn free_count() -> usize {
    GLOBAL_FREE_COUNT.load(Ordering::SeqCst)
}
pub fn run() {
    let mut res = Resource::new(42);
    unsafe {
        res.manual_release();
    }
}
fn main() {
    run();
}

/*
The following test oracle function invokes the code's run() function and then checks the global free count.
For the fixed implementation, the count is expected to be 1 (one free occurred via manual_release).
In the vulnerable version, the count will be 2 (manual_release and Drop both free the memory),
thereby causing the test to fail.
This test oracle is separate from the implementations.
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_free_protection() {
        // Reset the global counter. In a real test, consider isolation between runs.
        GLOBAL_FREE_COUNT.store(0, Ordering::SeqCst);
        run();
        let count = free_count();
        // In the fixed version, only one free should have occurred.
        assert_eq!(count, 1, "Double free vulnerability detected: free_count = {}", count);
    }
}
