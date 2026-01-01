use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct Data {
    pub value: i32,
}

pub fn process() -> i32 {
    let ptr: *mut Data;
    unsafe {
        let boxed = Box::new(Data { value: 512 });
        ptr = Box::into_raw(boxed);
        thread::sleep(Duration::from_millis(100));
        let result = (*ptr).value;
        Box::from_raw(ptr); // Ensure the box is dropped after reading the value
        result
    }
}

fn main() {
    let result = process();
    println!("Result: {}", result);
}