#![allow(unused)]
use std::thread;
trait Operations {
    unsafe fn release(&mut self);
}
pub struct Manager {
    ptr: *mut i32,
    is_freed: bool,
}
impl Manager {
    pub unsafe fn initialize(value: i32) -> Self {
        let boxed = Box::new(value);
        Manager {
            ptr: Box::into_raw(boxed),
            is_freed: false,
        }
    }
    pub unsafe fn process(&mut self) {
        self.release();
    }
}
impl Operations for Manager {
    unsafe fn release(&mut self) {
        if self.is_freed {
            return; 
        }
        let _ = Box::from_raw(self.ptr);
        self.is_freed = true;
    }
}
impl Drop for Manager {
    fn drop(&mut self) {
        unsafe {
            if !self.is_freed {
                let _ = Box::from_raw(self.ptr);
                self.is_freed = true;
            }
        }
    }
}
pub fn simulate() {
    unsafe {
        let mut resource = Manager::initialize(100);
        resource.process();
    }
}
fn main() {
    simulate();
}

///////////////////////////
// Test Oracle Code
///////////////////////////
#[cfg(test)]
mod tests {
    use std::panic;

    // The simulate() function encapsulates the core logic.
    // For the vulnerable version, simulate() will panic due to double free.
    // For the fixed version, simulate() will complete without panicking.
    // Note: This test oracle is the same for both versions.
    #[test]
    fn test_simulate() {
        // Catch unwind to see if simulate() panics.
        let result = panic::catch_unwind(|| {
            // Call the simulation function from the main module.
            crate::simulate();
        });
        // For the vulnerable version, the test should fail (result.is_err()).
        // For the fixed version, the test should pass (result.is_ok()).
        assert!(result.is_ok(), "simulate() panicked, indicating a double free vulnerability");
    }
}
