#![allow(dead_code)]
use std::thread;
trait Action {
    fn execute(&self) -> i32;
}
struct Holder {
    ptr: *const i32,
}
impl Holder {
    fn new(value: i32) -> Self {
        let b = Box::new(value);
        let ptr = Box::into_raw(b);
        Holder { ptr }
    }
    unsafe fn deallocate(&mut self) {
        if !self.ptr.is_null() {
            let _ = Box::from_raw(self.ptr as *mut i32);
            self.ptr = std::ptr::null();
        }
    }
    fn read(&self) -> i32 {
        if self.ptr.is_null() {
            -1
        } else {
            unsafe { *self.ptr }
        }
    }
}
impl Action for Holder {
    fn execute(&self) -> i32 {
        self.read()
    }
}
fn process(holder: &Holder) -> i32 {
    holder.execute()
}
fn main() {
    let mut resource = Holder::new(12345);
    let handle = thread::spawn(|| {
        let dummy = Box::new(99999);
        drop(dummy);
    });
    handle.join().unwrap();
    unsafe {
        resource.deallocate();
    }
    let res = process(&resource);
    println!("Result: {}", res);
}


//////////////////////////////////////////
// Test Oracle for Use-After-Free Issue //
//////////////////////////////////////////
#[cfg(test)]
mod tests {
    // Import all definitions from the parent module.
    use super::*;

    #[test]
    fn test_dangling_access() {
        let mut resource = Holder::new(12345);
        unsafe {
            resource.deallocate();
        }
        let result = process(&resource);
        // In the fixed version, the read function returns -1 after deallocation.
        // In the vulnerable version, the behavior is undefined and almost never -1.
        assert_eq!(result, -1, "The resource was accessed after being freed!");
    }
}
