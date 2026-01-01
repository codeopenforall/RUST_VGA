use std::sync::atomic::{AtomicBool, Ordering};
static mut FREED: bool = false;

struct Data {
    value: u32,
}

unsafe fn free_data(ptr: *mut Data) {
    if FREED {
        panic!("Double free detected!");
    }
    Box::from_raw(ptr);
    FREED = true;
    // Uncommenting the following line would fix the vulnerability
    // FREED = false;
}

fn app_run() {
    let boxed = Box::new(Data { value: 42 });
    let raw = Box::into_raw(boxed);
    unsafe {
        free_data(raw);
        free_data(raw);
    }
}

fn main() {
    app_run();
}