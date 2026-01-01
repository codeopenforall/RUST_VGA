use std::sync::atomic::{AtomicUsize, Ordering};

static GLOBAL_COUNT: AtomicUsize = AtomicUsize::new(0);

pub struct Resource {
    raw: *mut u8,
}

impl Resource {
    pub fn new() -> Self {
        GLOBAL_COUNT.fetch_add(1, Ordering::SeqCst);
        Resource { raw: std::ptr::null_mut() }
    }

    pub fn free(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                std::ptr::drop_in_place(self.raw);
            }
            self.raw = std::ptr::null_mut();
            GLOBAL_COUNT.fetch_sub(1, Ordering::SeqCst);
        }
    }
}

pub fn process() -> Result<(), String> {
    let mut resource = Resource::new();
    // Simulate some processing
    resource.free();
    // Attempting to free again should not cause a double free
    resource.free();
    Ok(())
}