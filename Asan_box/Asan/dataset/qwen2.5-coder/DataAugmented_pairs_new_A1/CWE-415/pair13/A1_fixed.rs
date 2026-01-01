#![allow(unused)]
use std::sync::atomic::{AtomicBool, Ordering};
struct Data {
    value: i32,
}
impl Data {
    fn new(v: i32) -> Self {
        Data { value: v }
    }
}
static TRIGGERED: AtomicBool = AtomicBool::new(false);
pub fn run() -> i32 {
    let resource = Box::new(Data::new(10));
    let raw_ptr = Box::into_raw(resource);
    unsafe {
        let box_a = Box::from_raw(raw_ptr);
        if TRIGGERED.swap(true, Ordering::SeqCst) {
            panic!("double free detected");
        }
        let sum = box_a.value * 2; // Correct the logic to avoid double freeing
        sum
    }
}
fn main() {
    println!("Result: {}", run());
}