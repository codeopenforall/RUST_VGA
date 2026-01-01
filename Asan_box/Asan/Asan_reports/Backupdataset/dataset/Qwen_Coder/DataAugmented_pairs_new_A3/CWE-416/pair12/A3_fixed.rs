#![allow(unused_unsafe)]
use std::thread;
use std::sync::{Arc, Mutex};
trait Arithmetic {
    fn compute(&self) -> i32;
}
struct Container {
    data: i32,
}
impl Arithmetic for Container {
    fn compute(&self) -> i32 {
        self.data + 1
    }
}
fn process() -> i32 {
    let resource = Box::new(Container { data: 99 });
    let raw = Box::into_raw(resource);
    let result = unsafe { (*raw).compute() }; 
    unsafe {
        drop(Box::from_raw(raw)); 
    }
    result
}
fn main() {
    let val = process();
    println!("Result: {}", val);
    let _handle = thread::spawn(|| {
        let boxed = Box::new(Container { data: 50 });
        let raw_ptr = Box::into_raw(boxed);
        let wrong_use = unsafe { (*raw_ptr).compute() };
        println!("Thread result: {}", wrong_use);
        unsafe {
            drop(Box::from_raw(raw_ptr)); 
        }
    });
    thread::sleep(std::time::Duration::from_millis(50));
}