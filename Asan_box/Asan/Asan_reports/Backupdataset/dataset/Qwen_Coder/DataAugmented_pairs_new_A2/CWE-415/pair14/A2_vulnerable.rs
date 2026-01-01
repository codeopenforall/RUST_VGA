#![allow(unused)]
use std::sync::atomic::{AtomicBool, Ordering};
struct Data {
    value: u32,
}
static mut FREED: AtomicBool = AtomicBool::new(false);
unsafe fn free_data(ptr: *mut Data) {
    if FREED.load(Ordering::SeqCst) {
        panic!("Double free detected!");
    }
    Box::from_raw(ptr);
    FREED.store(true, Ordering::SeqCst);
}
pub fn app_run() {
    unsafe {
        FREED.store(false, Ordering::SeqCst);
    }
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