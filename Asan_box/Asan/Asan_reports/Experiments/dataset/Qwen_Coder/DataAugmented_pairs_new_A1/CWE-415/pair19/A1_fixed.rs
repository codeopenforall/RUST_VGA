use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

static FREED: AtomicUsize = AtomicUsize::new(0);

unsafe fn custom_release(ptr: *mut i32) {
    if FREED.fetch_add(1, Ordering::SeqCst) == 0 {
        drop(Box::from_raw(ptr));
    }
}

struct Resource {
    ptr: *mut i32,
}

unsafe impl Send for Resource {}
unsafe impl Sync for Resource {}

impl Resource {
    fn new(val: i32) -> Self {
        let boxed = Box::new(val);
        let ptr = Box::into_raw(boxed);
        Resource { ptr }
    }

    fn release(&self) {
        unsafe {
            custom_release(self.ptr);
        }
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        unsafe {
            custom_release(self.ptr);
        }
    }
}

fn main() {
    let res = Arc::new(Resource::new(42));
    let res_clone = Arc::clone(&res);
    let handle = thread::spawn(move || {
        res_clone.release();
    });
    handle.join().unwrap();
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
