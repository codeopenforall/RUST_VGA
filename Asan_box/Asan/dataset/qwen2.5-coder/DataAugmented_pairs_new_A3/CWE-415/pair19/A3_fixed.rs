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