use std::sync::atomic::{AtomicBool, Ordering};

struct Resource {
    data: *mut i32,
    flag: *mut AtomicBool,
}

impl Resource {
    fn new(val: i32) -> Self {
        let data = Box::into_raw(Box::new(val));
        let flag = Box::into_raw(Box::new(AtomicBool::new(false)));
        Resource { data, flag }
    }

    fn clone(&self) -> Self {
        Resource {
            data: self.data,
            flag: self.flag,
        }
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        unsafe {
            if (*self.flag).compare_and_swap(false, true, Ordering::SeqCst) == false {
                let _ = Box::from_raw(self.data);
                let _ = Box::from_raw(self.flag);
            } else {
                panic!("Double free detected");
            }
        }
    }
}

fn main() {
    let resource1 = Resource::new(42);
    let resource2 = resource1.clone();

    drop(resource1);
    drop(resource2); // This will cause a double free and panic
}