use std::sync::atomic::{AtomicUsize, Ordering};
static FREE_COUNT: AtomicUsize = AtomicUsize::new(0);
pub struct Resource {
    data: *mut i32,
}
impl Resource {
    pub unsafe fn new(val: i32) -> Self {
        let boxed = Box::new(val);
        let ptr = Box::into_raw(boxed);
        Self { data: ptr }
    }
    pub unsafe fn release(&mut self) {
        if !self.data.is_null() {
            FREE_COUNT.fetch_add(1, Ordering::SeqCst);
            let _ = Box::from_raw(self.data);
        }
    }
}
impl Drop for Resource {
    fn drop(&mut self) {
        unsafe {
            if !self.data.is_null() {
                FREE_COUNT.fetch_add(1, Ordering::SeqCst);
                let _ = Box::from_raw(self.data);
            }
        }
    }
}
fn main() {
    unsafe {
        let mut res = Resource::new(42);
        res.release();
    }
}


//////////////////////////////////////////////////////////////////////////////////////////////////////
// Test Oracle: This test checks the number of free operations performed. In the vulnerable
// implementation, both the manual deallocation and the Drop implementation free the memory,
// resulting in FREE_COUNT equal to 2. In the fixed implementation, the pointer is nullified after
// the first free so that the Drop implementation does not free it again, resulting in FREE_COUNT equal to 1.
// The test fails for the vulnerable version and passes for the fixed version.
//////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::Ordering;

    #[test]
    fn test_double_free_detection() {
        // Reset the free counter before executing.
        FREE_COUNT.store(0, Ordering::SeqCst);
        
        // Execute the main logic.
        unsafe {
            let mut res = Resource::new(42);
            res.release();
        }
        
        // For the vulnerable implementation, the counter reaches 2.
        // For the fixed version, it remains 1.
        assert_eq!(
            FREE_COUNT.load(Ordering::SeqCst),
            1,
            "Double free detected: unexpected number of deallocations"
        );
    }
}
