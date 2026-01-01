#![allow(unused_unsafe)]
use std::sync::{Arc, Mutex};
use std::thread;

struct Resource {
    value: i32,
}

impl Resource {
    fn compute(&self) -> i32 {
        self.value * 2
    }
}

fn process() -> i32 {
    let resource = Box::new(Resource { value: 50 });
    let raw = Box::into_raw(resource);
    unsafe {
        let _ = Box::from_raw(raw); 
    }
    let result = unsafe { (*raw).compute() }; 
    result
}

fn main() {
    let boxed = Box::new(Resource { value: 75 });
    let _handle = thread::spawn(|| {
        let raw_ptr = Box::into_raw(boxed);
        unsafe {
            let _ = Box::from_raw(raw_ptr); 
        }
        let wrong_use = unsafe { (*raw_ptr).compute() };
        println!("Thread result: {}", wrong_use);
    });
}