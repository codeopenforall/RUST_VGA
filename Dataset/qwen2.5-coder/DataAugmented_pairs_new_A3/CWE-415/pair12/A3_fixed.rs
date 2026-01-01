use std::ptr;

struct Resource;

static mut FREED: bool = false;

unsafe fn deallocate_once(ptr: *mut Resource) {
    if !FREED {
        ptr::drop_in_place(ptr);
        FREED = true;
    }
}

fn main() {
    unsafe {
        let resource = Box::new(Resource);
        let raw_ptr = Box::into_raw(resource);
        deallocate_once(raw_ptr);
        deallocate_once(raw_ptr); // This should not cause a double free
    }
}