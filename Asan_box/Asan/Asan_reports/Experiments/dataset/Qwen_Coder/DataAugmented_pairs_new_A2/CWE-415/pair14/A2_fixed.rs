#![allow(unused)]
use std::ptr;
struct Data {
    value: u32,
}
struct Owner {
    ptr: *mut Data,
}
impl Owner {
    fn new(val: u32) -> Self {
        let boxed = Box::new(Data { value: val });
        Owner { ptr: Box::into_raw(boxed) }
    }
    unsafe fn release(&mut self) {
        if !self.ptr.is_null() {
            Box::from_raw(self.ptr);
            self.ptr = ptr::null_mut();
        }
    }
}
impl Drop for Owner {
    fn drop(&mut self) {
        unsafe {
            if !self.ptr.is_null() {
                Box::from_raw(self.ptr);
                self.ptr = ptr::null_mut();
            }
        }
    }
}
pub fn app_run() {
    let mut owner = Owner::new(42);
    unsafe {
        owner.release();
    }
}
fn main() {
    app_run();
}

//////////////////////////////////////////////
// Test Oracle: Validate Memory Deallocation Integrity
// The test calls app_run() and asserts no panic occurs.
// Under the vulnerable version, the double free would trigger a panic,
// while the corrected version runs to completion.
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::panic;
    // Import app_run from the module under test.
    use crate::app_run;

    #[test]
    fn test_memory_release() {
        let result = panic::catch_unwind(|| {
            app_run();
        });
        // The absence of a panic indicates the vulnerability has been fixed.
        assert!(result.is_ok(), "app_run panicked due to double free vulnerability.");
    }
}
