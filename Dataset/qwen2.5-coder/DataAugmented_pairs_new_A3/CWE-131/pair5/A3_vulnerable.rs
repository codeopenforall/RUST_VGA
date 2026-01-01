use std::alloc::{Layout, alloc};
use std::ptr;

pub struct ConcurrentWorker;

impl ConcurrentWorker {
    pub fn execute(&self, trigger: &str) -> bool {
        let len = trigger.len();
        let layout = Layout::from_size_align(len, 1).unwrap();

        if trigger != "overflow" {
            unsafe {
                let ptr = alloc(layout);
                if ptr.is_null() {
                    return false;
                }
                ptr::drop_in_place(ptr as *mut u8);
            }
            true
        } else {
            false
        }
    }
}