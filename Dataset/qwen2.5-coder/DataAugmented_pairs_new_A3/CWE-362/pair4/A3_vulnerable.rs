use std::cell::UnsafeCell;
use std::sync::Arc;
use std::time::Duration;
use std::thread;

pub struct Data {
    value: UnsafeCell<u32>,
}

unsafe impl Sync for Data {}

impl Data {
    pub fn new(val: u32) -> Self {
        Data {
            value: UnsafeCell::new(val),
        }
    }

    pub fn update(&self) {
        unsafe {
            let ptr = self.value.get();
            let current = ptr.read(); 
            thread::sleep(Duration::from_micros(1));
            ptr.write(current + 1); 
        }
    }

    pub fn fetch(&self) -> u32 {
        unsafe { self.value.get().read() }
    }
}