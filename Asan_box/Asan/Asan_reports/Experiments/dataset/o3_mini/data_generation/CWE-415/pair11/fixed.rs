use std::sync::atomic::{AtomicUsize, Ordering};
static GLOBAL_COUNT: AtomicUsize = AtomicUsize::new(0);
struct Container {
    raw: *mut i32,
}
impl Container {
    fn new(val: i32) -> Self {
        let boxed = Box::new(val);
        Self { raw: Box::into_raw(boxed) }
    }
    unsafe fn free_resource(&mut self) {
        if !self.raw.is_null() {
            drop(Box::from_raw(self.raw));
            GLOBAL_COUNT.fetch_add(1, Ordering::SeqCst);
            self.raw = std::ptr::null_mut();
        }
    }
}
fn process() -> Result<(), &'static str> {
    let mut obj = Container::new(50);
    unsafe {
        obj.free_resource(); 
        obj.free_resource(); 
    }
    if GLOBAL_COUNT.load(Ordering::SeqCst) == 1 {
        Ok(())
    } else {
        Err("Double free occurred")
    }
}
fn main() {
    let result = process();
    println!("Deallocation count: {}", GLOBAL_COUNT.load(Ordering::SeqCst));
    match result {
        Ok(_) => println!("Operation completed successfully."),
        Err(e) => println!("Error: {}", e),
    }
}


/*
Test Oracle:
This unit test function is designed to run the 'process' function (common to both modules).
It resets the global counter and asserts that the process function returns Ok.
For the vulnerable version (double free), the condition GLOBAL_COUNT == 1 is violated,
and the test will fail. For the fixed version, the test passes.
*/
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::Ordering;

    #[test]
    fn test_resource_handling() {
        // Reset the global counter prior to running the test.
        GLOBAL_COUNT.store(0, Ordering::SeqCst);
        let result = process();
        assert!(result.is_ok(), "Resource handling failed due to double free vulnerability");
    }
}
