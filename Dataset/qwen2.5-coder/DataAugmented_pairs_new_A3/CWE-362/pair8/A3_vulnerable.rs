use std::{cell::UnsafeCell, sync::Arc, thread, time::Duration};

pub struct Resource {
    flag: UnsafeCell<bool>,
    init_count: UnsafeCell<u32>,
}

unsafe impl Sync for Resource {}

impl Resource {
    pub fn new() -> Self {
        Resource {
            flag: UnsafeCell::new(false),
            init_count: UnsafeCell::new(0),
        }
    }

    pub fn process(&self) {
        unsafe {
            if !*self.flag.get() {
                thread::sleep(Duration::from_millis(10));
                *self.init_count.get() = *self.init_count.get() + 1;
                *self.flag.get() = true;
            }
        }
    }

    pub fn get_count(&self) -> u32 {
        unsafe { *self.init_count.get() }
    }
}