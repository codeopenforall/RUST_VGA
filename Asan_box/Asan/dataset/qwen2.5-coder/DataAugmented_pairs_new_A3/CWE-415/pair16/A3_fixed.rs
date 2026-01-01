use std::sync::atomic::{AtomicUsize, Ordering};

static GLOBAL_DROP: AtomicUsize = AtomicUsize::new(0);

struct Resource;

impl Drop for Resource {
    fn drop(&mut self) {
        GLOBAL_DROP.fetch_add(1, Ordering::SeqCst);
    }
}

fn process_resource() {
    let raw = Box::into_raw(Box::new(Resource));
    unsafe {
        // Ensure the resource is dropped properly
        let _holder = Box::from_raw(raw);
    }
}