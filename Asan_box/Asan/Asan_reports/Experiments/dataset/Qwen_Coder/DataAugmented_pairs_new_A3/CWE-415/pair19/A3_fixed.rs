use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};

static FREED: AtomicUsize = AtomicUsize::new(0);

unsafe fn custom_release(ptr: usize) {
    // Simulate resource release
    FREED.fetch_add(1, Ordering::SeqCst);
}

struct Resource {
    ptr: usize,
    freed: AtomicBool,
}

impl Resource {
    fn new(ptr: usize) -> Self {
        Resource { 
            ptr, 
            freed: AtomicBool::new(false) 
        }
    }

    fn release(&self) {
        if !self.freed.swap(true, Ordering::SeqCst) {
            unsafe {
                custom_release(self.ptr);
            }
        }
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        if !self.freed.load(Ordering::SeqCst) {
            unsafe {
                custom_release(self.ptr);
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Test Oracle Function
// To be compiled and run separately from the vulnerable and corrected code.
////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::panic;
    use std::sync::atomic::Ordering;
    // Bring Resource and FREED into scope from the module under test.
    use super::*;

    #[test]
    fn test_resource_deallocation() {
        // Reset the global free counter before the test.
        FREED.store(0, Ordering::SeqCst);
        // Create a Resource with a distinct value.
        let resource = Resource::new(100);
        // Catch any panic that results from a double free.
        let result = panic::catch_unwind(|| {
            // Calling release() should free the resource.
            resource.release();
            // When resource goes out of scope, Drop is executed.
        });
        // In the vulnerable version, a panic is triggered when Drop performs a second free.
        // In the corrected version, the panic does not occur.
        assert!(result.is_ok(), "Resource deallocation must not result in a double free panic");
    }
}
