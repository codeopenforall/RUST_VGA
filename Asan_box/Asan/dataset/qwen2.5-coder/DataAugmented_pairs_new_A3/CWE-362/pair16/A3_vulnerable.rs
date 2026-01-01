use std::cell::UnsafeCell;
use std::thread;
use std::time::Duration;

pub struct Core {
    flag: UnsafeCell<bool>,
}

impl Core {
    pub fn new() -> Self {
        Self { flag: UnsafeCell::new(true) }
    }

    pub fn try_use(&self) -> bool {
        unsafe {
            if *self.flag.get() {
                thread::sleep(Duration::from_micros(10));
                *self.flag.get() = false;
                return true;
            }
        }
        false
    }
}

unsafe impl Sync for Core {}