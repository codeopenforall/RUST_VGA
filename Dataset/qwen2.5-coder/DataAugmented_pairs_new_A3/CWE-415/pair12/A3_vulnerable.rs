use std::cell::RefCell;

struct Resource;

static mut FREED: bool = false;

unsafe fn deallocate(ptr: *mut Resource) {
    if FREED {
        panic!("Double free detected");
    } else {
        FREED = true;
        // Intentionally causing a double free by calling deallocate twice
        deallocate(ptr);
        deallocate(ptr);
    }
}

fn main() {
    let resource = Box::new(Resource);
    let raw_ptr = Box::into_raw(resource);
    unsafe {
        deallocate(raw_ptr);
    }
}