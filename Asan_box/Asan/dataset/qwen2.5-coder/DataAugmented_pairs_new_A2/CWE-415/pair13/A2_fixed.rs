#![allow(unused)]
struct Data {
    value: i32,
}
impl Data {
    fn new(v: i32) -> Self {
        Data { value: v }
    }
}
pub fn run() -> i32 {
    let resource = Box::new(Data::new(10));
    let raw_ptr = Box::into_raw(resource);
    unsafe {
        let box_a = Box::from_raw(raw_ptr);
        let result = box_a.value * 2;
        std::mem::forget(box_a); // Prevent double free
        result
    }
}
fn main() {
    println!("Result: {}", run());
}