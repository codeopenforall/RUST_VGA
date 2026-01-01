use std::thread;
use std::time::Duration;

struct Resource {
    ptr: *mut i32,
}

impl Resource {
    fn new(val: i32) -> Self {
        let b = Box::new(val);
        let ptr = Box::into_raw(b);
        Resource { ptr }
    }

    unsafe fn dispose(&mut self) {
        let _ = Box::from_raw(self.ptr);
        self.ptr = std::ptr::null_mut(); // Ensure the pointer is null after disposal
    }

    unsafe fn read(&self) -> i32 {
        if self.ptr.is_null() {
            panic!("Attempted to read from a disposed resource");
        }
        *self.ptr
    }
}

pub fn execute() -> i32 {
    let mut res = Resource::new(123);
    unsafe {
        let value = res.read(); // Read before disposing
        res.dispose();
        let _dummy = Box::new(999);
        value
    }
}

fn main() {
    let result = execute();
    println!("Value: {}", result);
}